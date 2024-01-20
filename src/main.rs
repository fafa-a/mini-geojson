use clap::Parser;
use log::{error, info};
use mini_geojson::args::Args;
use mini_geojson::file_operations::{
    calculate_size_reduction, get_file_size, handle_geojson_processing, handle_output_path,
};
use size::Size;

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    info!("Starting program with arguments: {:?}", args);

    let output_path = match handle_output_path(&args) {
        Ok(path) => path,
        Err(e) => {
            error!("Error handling output path: {}", e);
            return;
        }
    };

    let input_path_clone = args.input.clone();
    let output_path_clone = output_path.clone();

    match handle_geojson_processing(args, output_path) {
        Ok(()) => {
            println!("Program completed successfully.");
        }
        Err(e) => {
            error!("Error processing GeoJSON: {}", e);
        }
    }

    let original_size = get_file_size(&input_path_clone).unwrap();
    let minified_size = get_file_size(&output_path_clone.to_string_lossy()).unwrap();
    let original_size_formatted = Size::from_bytes(original_size).to_string();
    let minified_size_formatted = Size::from_bytes(minified_size).to_string();

    let reduction_percentage = calculate_size_reduction(original_size, minified_size);
    println!(
        "File size reduced by {:.2}% (from {} bytes to {} bytes)",
        reduction_percentage, original_size_formatted, minified_size_formatted
    );
}
