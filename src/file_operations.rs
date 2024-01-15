use crate::args::Args;
use crate::geo_operations::truncate_coordinate_in_array;
use log::{debug, error, info};
use serde_json::{from_reader, to_writer, to_writer_pretty, Value};
use std::fs::{self, File};
use std::io;
use std::io::Write;
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
    Json(#[from] serde_json::Error),

    // Error from the program
    // Variants for handle_output_path
    #[error("Invalid filename in the input path")]
    InvalidFilename,

    #[error("File already exists and overwrite not allowed")]
    FileExists,

    #[error("Failed to create the output directory: {0}")]
    DirectoryCreationError(String),
}

pub fn read_json_file<P: AsRef<Path>>(file_path: P) -> Result<Value, MyError> {
    info!("Reading file: {:?}", file_path.as_ref());
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => {
            error!(
                "Failed to open file: {:?}, error: {}",
                file_path.as_ref(),
                e
            );
            return Err(MyError::Io(e));
        }
    };
    let reader = io::BufReader::new(file);

    from_reader(reader).map_err(|e| {
        error!(
            "Failed to read JSON from file: {:?}, error: {}",
            file_path.as_ref(),
            e
        );
        MyError::Json(e)
    })
}

pub fn write_geojson_file(
    geojson: &Value,
    output_file: &mut File,
    pretty: bool,
) -> Result<(), MyError> {
    info!("Writing GeoJSON file, pretty: {}", pretty);

    let write_result = if pretty {
        to_writer_pretty(&mut *output_file, geojson)
    } else {
        to_writer(&mut *output_file, geojson)
    };

    write_result.map_err(|e| {
        error!("Failed to write GeoJSON, error: {}", e);
        MyError::Json(e)
    })?;

    output_file.flush().map_err(|e| {
        error!("Failed to flush output file, error: {}", e);
        MyError::Io(e)
    })?;

    Ok(())
}

pub fn process_geojson(
    geojson: &mut Value,
    decimal: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Processing GeoJSON, with decimal precision: {}", decimal);
    if let Some(features) = geojson["features"].as_array_mut() {
        for feature in features.iter_mut() {
            if let Some(geometry) = feature["geometry"].as_object_mut() {
                if let Some(coords) = geometry.get_mut("coordinates") {
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

    process_geojson(&mut geojson, args.decimal)?;
    info!("GeoJSON processed successfully.");

    let mut file = File::create(&output_path)?;
    write_geojson_file(&geojson, &mut file, args.pretty)?;
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

pub fn is_geojson(parsed_json: &Value) -> bool {
    debug!("Checking if parsed JSON is GeoJSON: {}", parsed_json);
    let is_geojson = if let Some(geometry) = parsed_json.get("geometry") {
        geometry
            .get("coordinates")
            .and_then(|c| c.as_array())
            .map_or(false, |coords| !coords.is_empty())
    } else {
        false
    };
    info!("Is GeoJSON: {}", is_geojson);
    is_geojson
}
