use clap::Parser;
use log::{error, info};
use mini_geojson::args::Args;
use mini_geojson::file_operations::{handle_geojson_processing, handle_output_path};
use mini_geojson::metrics::calculate_and_display_size_reduction;

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

    match handle_geojson_processing(&args, &output_path) {
        Ok(()) => {
            println!("Program completed successfully.");
            calculate_and_display_size_reduction(&args.input, &output_path);
        }
        Err(e) => {
            error!("Error processing GeoJSON: {}", e);
        }
    }
}
