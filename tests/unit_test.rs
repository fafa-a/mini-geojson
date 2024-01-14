use clap::Parser;
use mini_geojson::args::Args; // Import Args for testing
use mini_geojson::file_operations::{
    add_prefix_to_filename, extract_filename_from_path, is_geojson, read_json_file,
};

use mini_geojson::geo_operations::truncate_coordinate_in_array;
use serde_json::json;

#[test]
fn test_base_arg_parsing_with_short_arg() {
    let args = Args::parse_from(["mini-geojson", "-i", "input.geojson", "-d", "3"]);
    assert_eq!(args.input, "input.geojson");
    assert_eq!(args.output, "./output/");
    assert_eq!(args.decimal, 3);
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
    assert_eq!(args.decimal, 3);
    assert!(args.overwrite);
    assert!(args.pretty);
}

#[test]
fn test_base_arg_parsing_with_long_arg() {
    let args = Args::parse_from(["mini-geojson", "--input", "input.geojson", "--decimal", "3"]);
    assert_eq!(args.input, "input.geojson");
    assert_eq!(args.output, "./output/");
    assert_eq!(args.decimal, 3);
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
    assert_eq!(args.decimal, 3);
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

#[test]
fn test_truncate_coord() {
    let mut coordinates = json!([1.234567, 2.345678]);
    let decimal = 2;

    truncate_coordinate_in_array(&mut coordinates, decimal);

    assert_eq!(coordinates, json!([1.23, 2.35]));
}

#[test]
fn test_truncate_coordinate_in_array_of_array() {
    let mut coordinates = json!([[1.234567, 2.345678], [4.567890, 5.678901]]);
    let decimal = 2;

    truncate_coordinate_in_array(&mut coordinates, decimal);

    assert_eq!(coordinates, json!([[1.23, 2.35], [4.57, 5.68]]));
}

#[test]
fn truncate_coordinate_in_geojson_geometry() {
    let mut coordinates = json!([
        [
            [1.234567, 2.345678],
            [4.567890, 5.678901],
            [7.890123, 8.901234],
            [1.234567, 2.345678]
        ],
        [
            [1.234567, 2.345678],
            [4.567890, 5.678901],
            [7.890123, 8.901234],
            [1.234567, 2.345678]
        ],
        [
            [1.234567, 2.345678],
            [4.567890, 5.678901],
            [7.890123, 8.901234],
            [1.234567, 2.345678]
        ]
    ]);
    let decimal = 2;
    truncate_coordinate_in_array(&mut coordinates, decimal);
    assert_eq!(
        coordinates,
        json!([
            [[1.23, 2.35], [4.57, 5.68], [7.89, 8.90], [1.23, 2.35]],
            [[1.23, 2.35], [4.57, 5.68], [7.89, 8.90], [1.23, 2.35]],
            [[1.23, 2.35], [4.57, 5.68], [7.89, 8.90], [1.23, 2.35]]
        ])
    );
}
