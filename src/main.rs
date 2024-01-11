mod file_operations;

use std::fs;

use clap::Parser;
use mini_geojson::args::Args;
use mini_geojson::file_operations::{extract_filename_from_path, is_geojson};
use serde_json::Value;

fn main() {
    let args = Args::parse();
    let (input, output, decimal) = (args.input, args.output, args.decimal);
    println!(
        "Input file: {} \nOutput file: {} \nDecimal places to truncate: {}",
        input, output, decimal
    );

    let filename = extract_filename_from_path(&input);
    if let Some(filename) = filename {
        println!("Filename: {:?}", filename)
    } else {
        println!("No filename found")
    }

    let content = fs::read_to_string(input).expect("Unable to read the file");
    let parsed_json: Value = serde_json::from_str(&content).unwrap();
    let geojson = is_geojson(&parsed_json);
    if geojson {
        println!("This is GeoJSON");
        println!("parsed_json: {:?}", parsed_json);
    }
}
