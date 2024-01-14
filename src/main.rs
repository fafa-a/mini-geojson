use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use mini_geojson::args::Args;
use mini_geojson::file_operations::{extract_filename_from_path, handle_geojson_processing};

fn main() {
    let args = Args::parse();
    println!(
        "Input file: {} \nOutput file: {} \nDecimal places to keep: {} \nOverwrite: {}",
        args.input, args.output, args.decimal, args.overwrite
    );

    let mut output_path = PathBuf::from(&args.output);

    if output_path == PathBuf::from("./output/") {
        if let Some(filename) = extract_filename_from_path(&args.input) {
            output_path.push(format!("min_{}", filename));
        } else {
            println!("No valid filename found in the input path.");
            return;
        }
    }

    if Path::new(&output_path).exists() && !args.overwrite {
        println!("File already exists. Use --overwrite to overwrite.");
        return;
    }
    println!("Output file: {:?}", &output_path);

    if let Some(parent) = Path::new(&output_path).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create the output directory: {}", e);
            return;
        }
    }

    handle_geojson_processing(args, output_path).unwrap_or_else(|e| {
        eprintln!("Error processing GeoJSON: {}", e);
    });
}
