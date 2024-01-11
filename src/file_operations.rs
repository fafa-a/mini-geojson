use serde_json::Value;

use std::path::Path;

enum FileExtension {
    GeoJson,
    Json,
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
    //println!("Content: {}", content);

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
