use clap::Parser;
use mini_geojson::args::Args; // Import Args for testing
use mini_geojson::file_operations::{
    add_prefix_to_filename, extract_filename_from_path, is_geojson,
};

#[test]
fn test_arg_parsing() {
    let args = Args::parse_from(["mini-geojson", "-i", "input.geojson", "-d", "3"]);

    assert_eq!(args.input, "input.geojson");
    assert_eq!(args.output, "min_input_filename.geojson"); // Default value
    assert_eq!(args.decimal, 3);
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
fn test_is_geojson() {
    let is_geojson = is_geojson("data/test-geojson-true.geojson");
    assert_eq!(is_geojson, true);
}

#[test]
fn test_is_not_geojson() {
    let is_geojson = is_geojson("data/test-geojson-false.geojson");
    assert_eq!(is_geojson, false);
}

#[test]
fn test_is_geojson_with_geometry_but_no_coords() {
    let is_geojson = is_geojson("data/test-geojson-geometry-no-coords.geojson");
    assert_eq!(is_geojson, false);
}
