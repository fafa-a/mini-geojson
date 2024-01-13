use crate::geo_operations::truncate_coordinate_in_array;

use serde_json::{from_str, Value};
use std::fs;
use std::path::Path;

enum FileExtension {
    GeoJson,
    Json,
}

pub fn read_json_file(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let parsed_json: Value = from_str(&content)?;

    Ok(parsed_json)
}

pub fn write_geojson_file(
    geojson: &Value,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let geojson_string = serde_json::to_string_pretty(geojson)?;
    fs::write(output_file, geojson_string)?;

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
