use crate::args::Args;
use crate::geo_operations::truncate_coordinate_in_array;

use serde_json::{from_reader, to_writer, to_writer_pretty, Value};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

enum FileExtension {
    GeoJson,
    Json,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn read_json_file<P: AsRef<Path>>(file_path: P) -> Result<Value, MyError> {
    let file = File::open(file_path).map_err(MyError::Io)?;
    let reader = io::BufReader::new(file);

    from_reader(reader).map_err(MyError::Json)
}

pub fn write_geojson_file(
    geojson: &Value,
    output_file: &mut File,
    pretty: bool,
) -> Result<(), MyError> {
    if pretty {
        to_writer_pretty(&mut *output_file, geojson).map_err(MyError::Json)?;
    } else {
        to_writer(&mut *output_file, geojson).map_err(MyError::Json)?;
    }

    output_file.flush().map_err(MyError::Io)?;
    Ok(())
}

pub fn process_geojson(
    geojson: &mut Value,
    decimal: usize,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut geojson = read_json_file(&args.input)?;

    process_geojson(&mut geojson, args.decimal)?;
    println!("GeoJSON processed successfully.");

    let mut file = File::create(&output_path)?;
    write_geojson_file(&geojson, &mut file, args.pretty)?;
    println!("GeoJSON written successfully in {:?}", output_path);

    Ok(())
}
fn extract_file_extension(ext: &str) -> Option<FileExtension> {
    match ext {
        "geojson" => Some(FileExtension::GeoJson),
        "json" => Some(FileExtension::Json),
        _ => None,
    }
}

/// Extract the filename from a path (example: "/home/user/input.geojson" -> "input.geojson")
/// return the filename if it is already a filename (example: "input.geojson" -> "input.geojson")
/// Or return None if the path does not contain a filename (example: "/home/user/")
/// or if the filename is empty (example: "/home/user/.geojson")
/// or if the filename is "/" (example: "/home/user/")
pub fn extract_filename_from_path(path: &str) -> Option<String> {
    let file_path = Path::new(path);

    if let Some(file_name) = file_path.file_name()?.to_str() {
        if let Some(ext) = file_path.extension()?.to_str() {
            println!("Extension: {}", ext);
            if extract_file_extension(ext).is_some() {
                return Some(file_name.to_string());
            }
        }
    }
    None
}

/// Add a prefix to a filename (example: "input.geojson" + "min_" = "min_input.geojson")
pub fn add_prefix_to_filename(filename: &str, prefix: &str) -> String {
    format!("{}{}", prefix, filename)
}

pub fn is_geojson(parsed_json: &Value) -> bool {
    println!("Parsed JSON: {}", parsed_json);
    // Check if there is a "geometry" property
    if let Some(geometry) = parsed_json.get("geometry") {
        // Check if the "geometry" property has coordinates
        if let Some(coordinates) = geometry.get("coordinates") {
            // Check if the coordinates are not empty
            return !coordinates.as_array().unwrap().is_empty();
        }
    }
    false
}
