use crate::args::Args;
use crate::geo_operations::truncate_coordinate_in_array;
use log::{debug, error, info};
use sonic_rs::{
    from_str, to_writer, to_writer_pretty, JsonValueMutTrait, JsonValueTrait, Value as SonicValue,
};
use std::fs::{self, File};
use std::io::{self};
use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug)]
enum FileExtension {
    GeoJson,
    Json,
}

#[derive(Error, Debug)]
pub enum MyError {
    // Error from libraries
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] sonic_rs::Error),

    // Error from the program
    // Variants for handle_output_path         1)
    #[error("Invalid filename in the input path")]
    InvalidFilename,

    #[error("File already exists and overwrite not allowed")]
    FileExists,

    #[error("Failed to create the output directory: {0}")]
    DirectoryCreationError(String),
}

pub fn get_file_size(file_path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len())
}

pub fn calculate_size_reduction(original_size: u64, minified_size: u64) -> f64 {
    let reduction = original_size as f64 - minified_size as f64;
    (reduction / original_size as f64) * 100.0
}

pub fn read_json_file<P: AsRef<Path>>(file_path: P) -> Result<SonicValue, MyError> {
    info!("Reading file: {:?}", file_path.as_ref());

    let file_content = fs::read_to_string(&file_path).map_err(|e| {
        error!(
            "Failed to read file: {:?}, error: {}",
            file_path.as_ref(),
            e
        );
        MyError::Io(e)
    })?;

    from_str(&file_content).map_err(|e| {
        error!(
            "Failed to parse JSON from file: {:?}, error: {}",
            file_path.as_ref(),
            e
        );
        MyError::Json(e)
    })
}
pub fn write_geojson_file(
    geojson: &SonicValue,
    mut output_file: File,
    pretty: bool,
) -> IoResult<()> {
    info!("Writing GeoJSON file, pretty: {}", pretty);
    let mut buffer = Vec::new();

    if pretty {
        to_writer_pretty(&mut buffer, geojson)?;
    } else {
        to_writer(&mut buffer, geojson)?;
    };

    output_file.write_all(&buffer)?;

    output_file.flush()?;
    Ok(())
}

pub fn process_geojson(
    geojson: &mut SonicValue,
    decimal: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Processing GeoJSON, with decimal precision: {}", decimal);
    if let Some(features) = geojson.get_mut("features").as_array_mut() {
        for feature in features.iter_mut() {
            if let Some(geometry) = feature.get_mut("geometry").as_object_mut() {
                if let Some(coords) = geometry.get_mut(&"coordinates".to_string()) {
                    truncate_coordinate_in_array(coords, decimal);
                }
            }
        }
    }

    Ok(())
}

pub fn handle_geojson_processing(
    args: Args,
    output_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Handling GeoJSON processing for file: {:?}", args.input);
    let mut geojson = read_json_file(&args.input)?;

    if let Some(decimal) = args.decimal {
        process_geojson(&mut geojson, decimal)?;
    }
    info!("GeoJSON processed successfully.");

    let file = File::create(&output_path)?;
    write_geojson_file(&geojson, file, args.pretty)?;
    info!("GeoJSON written successfully to {:?}", output_path);

    Ok(())
}

pub fn handle_output_path(args: &Args) -> Result<PathBuf, MyError> {
    let sanitized_output = sanitize_output_path(&args.output);
    let mut output_path = PathBuf::from(sanitized_output);

    if output_path == PathBuf::from("./output/") || args.output.ends_with('/') {
        if let Some(filename) = extract_filename_from_path(&args.input) {
            output_path.push(format!("min_{}", filename));
            info!(
                "Output path set to default, filename adjusted: {:?}",
                output_path
            );
        } else {
            error!("Invalid filename in input path: {}", args.input);
            return Err(MyError::InvalidFilename);
        }
    }

    if Path::new(&output_path).exists() && !args.overwrite {
        error!(
            "Output path already exists and overwrite is not allowed: {:?}",
            output_path
        );
        return Err(MyError::FileExists);
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            error!("Failed to create directory: {:?}, error: {}", parent, e);
            MyError::DirectoryCreationError(e.to_string())
        })?;
    }

    info!("Output path: {:?}", output_path);
    Ok(output_path)
}

fn sanitize_output_path(output_path: &str) -> String {
    let sanitized: String = output_path
        .chars()
        .filter(|&c| !matches!(c, '?' | '%' | '*' | ':' | '|' | '"' | '<' | '>'))
        .map(|c| if c == ' ' { '_' } else { c })
        .collect();

    let trimmed = sanitized.trim_end_matches(|c| c == '-' || c == '_');

    if trimmed.ends_with(".geojson") {
        trimmed.to_string()
    } else if trimmed.ends_with(".json") {
        format!("{}.geojson", trimmed.trim_end_matches(".json"))
    } else {
        format!("{}.geojson", trimmed)
    }
}

fn extract_file_extension(ext: &str) -> Option<FileExtension> {
    info!("Extracting file extension: {}", ext);
    let extension = match ext {
        "geojson" => Some(FileExtension::GeoJson),
        "json" => Some(FileExtension::Json),
        _ => None,
    };

    debug!("Extracted extension: {:?}", extension);
    extension
}

/// Extract the filename from a path (example: "/home/user/input.geojson" -> "input.geojson")
/// return the filename if it is already a filename (example: "input.geojson" -> "input.geojson")
/// Or return None if the path does not contain a filename (example: "/home/user/")
/// or if the filename is empty (example: "/home/user/.geojson")
/// or if the filename is "/" (example: "/home/user/")
pub fn extract_filename_from_path(path: &str) -> Option<String> {
    debug!("Extracting filename from path: {}", path);
    let file_path = Path::new(path);

    if let Some(file_name) = file_path.file_name()?.to_str() {
        if let Some(ext) = file_path.extension()?.to_str() {
            debug!("Extension: {}", ext);
            if extract_file_extension(ext).is_some() {
                return Some(file_name.to_string());
            }
        }
    }
    info!("No valid filename extracted from path: {}", path);
    None
}

/// Add a prefix to a filename (example: "input.geojson" + "min_" = "min_input.geojson")
pub fn add_prefix_to_filename(filename: &str, prefix: &str) -> String {
    debug!("Adding prefix '{}' to filename '{}'", prefix, filename);
    let new_filename = format!("{}{}", prefix, filename);
    info!("New filename: {}", new_filename);
    new_filename
}

pub fn is_geojson(parsed_json: &SonicValue) -> bool {
    debug!("Checking if parsed JSON is GeoJSON: {}", parsed_json);
    let is_geojson = if let Some(geometry) = parsed_json.get("geometry") {
        if let Some(coordinates) = geometry.get("coordinates") {
            // Check if 'coordinates' is an array and not null or another type
            coordinates.is_array() && !coordinates.is_null()
        } else {
            false
        }
    } else {
        false
    };
    info!("Is GeoJSON: {}", is_geojson);
    is_geojson
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    // use serde_json::json;
    // use std::io::{Read, Seek, SeekFrom};
    // use tempfile::NamedTempFile;

    use super::*;
    #[test]
    fn test_base_arg_parsing_with_short_arg() {
        let args = Args::parse_from(["mini-geojson", "-i", "input.geojson", "-d", "3"]);
        assert_eq!(args.input, "input.geojson");
        assert_eq!(args.output, "./output/");
        assert_eq!(args.decimal, Some(3));
        assert!(!args.overwrite);
        assert!(!args.pretty);
    }

    #[test]
    fn test_all_args_parsing_with_short_arg() {
        let args = Args::parse_from([
            "mini-geojson",
            "-i",
            "input.geojson",
            "-o",
            "output.geojson",
            "-d",
            "3",
            "-O",
            "-p",
        ]);
        assert_eq!(args.input, "input.geojson");
        assert_eq!(args.output, "output.geojson");
        assert_eq!(args.decimal, Some(3));
        assert!(args.overwrite);
        assert!(args.pretty);
    }

    #[test]
    fn test_base_arg_parsing_with_long_arg() {
        let args = Args::parse_from(["mini-geojson", "--input", "input.geojson", "--decimal", "3"]);
        assert_eq!(args.input, "input.geojson");
        assert_eq!(args.output, "./output/");
        assert_eq!(args.decimal, Some(3));
        assert!(!args.overwrite);
        assert!(!args.pretty);
    }

    #[test]
    fn test_all_args_parsing_with_long_arg() {
        let args = Args::parse_from([
            "mini-geojson",
            "--input",
            "input.geojson",
            "--output",
            "output.geojson",
            "--decimal",
            "3",
            "--overwrite",
            "--pretty",
        ]);
        assert_eq!(args.input, "input.geojson");
        assert_eq!(args.output, "output.geojson");
        assert_eq!(args.decimal, Some(3));
        assert!(args.overwrite);
        assert!(args.pretty);
    }

    #[test]
    fn test_extract_filename_from_path_with_no_filename_given() {
        let filename = extract_filename_from_path("/home/user/");
        assert_eq!(filename, None);
    }

    #[test]
    fn test_extract_filename_from_path() {
        let filename = extract_filename_from_path("/home/user/input.geojson");
        assert_eq!(filename, Some("input.geojson".to_string()));
    }

    #[test]
    fn test_extract_filename_with_filename_for_path() {
        let filename = extract_filename_from_path("input.geojson");
        assert_eq!(filename, Some("input.geojson".to_string()));
    }

    #[test]
    fn test_add_prefix_to_filename() {
        let filename = add_prefix_to_filename("input.geojson", "min_");
        assert_eq!(filename, "min_input.geojson");
    }

    #[test]
    fn test_is_geometry_in_feature() {
        let file_path = "data/test-geojson-true.geojson";
        let parsed_json = read_json_file(file_path).unwrap();
        let is_geojson = is_geojson(&parsed_json);
        assert!(is_geojson);
    }

    #[test]
    fn test_is_not_geometry_in_feature() {
        let file_path = "data/test-geojson-false.geojson";
        let parsed_json = read_json_file(file_path).unwrap();
        let is_geojson = is_geojson(&parsed_json);
        assert!(!is_geojson);
    }

    #[test]
    fn test_is_geometry_in_feature_with_no_coords() {
        let file_path = "data/test-geojson-geometry-no-coords.geojson";
        let parsed_json = read_json_file(file_path).unwrap();
        let is_geojson = is_geojson(&parsed_json);
        assert!(!is_geojson);
    }

    //#[test]
    // fn test_no_whitespace_no_new_line() -> Result<(), io::Error> {
    //     let geojson = json!({
    //         "type": "FeatureCollection",
    //         "features": []
    //     });
    //
    //     let mut temp_file = NamedTempFile::new()?;
    //     to_writer(&mut temp_file, &geojson)?;
    //     temp_file.seek(SeekFrom::Start(0))?;
    //
    //     let mut min_geojson = String::new();
    //     temp_file.read_to_string(&mut min_geojson)?;
    //
    //     let min_geojson: Value = serde_json::from_str(&min_geojson)?;
    //     let minified_geojson = json!({"type":"FeatureCollection","features":[]});
    //     assert_eq!(minified_geojson, min_geojson);
    //
    //     Ok(())
    // }
}
