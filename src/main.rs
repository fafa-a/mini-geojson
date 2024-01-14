use clap::Parser;
use mini_geojson::args::Args;
use mini_geojson::file_operations::{handle_geojson_processing, handle_output_path};

fn main() {
    let args = Args::parse();
    println!(
        "Input file: {} \nOutput file: {} \nDecimal to keep: {} \nOverwrite: {}",
        args.input, args.output, args.decimal, args.overwrite
    );

    let output_path = match handle_output_path(&args) {
        Ok(path) => {
            println!("Output file: {:?}", path);
            path
        }
        Err(e) => {
            eprintln!("Error handling output path: {}", e);
            return;
        }
    };

    handle_geojson_processing(args, output_path).unwrap_or_else(|e| {
        eprintln!("Error processing GeoJSON: {}", e);
    });
}
