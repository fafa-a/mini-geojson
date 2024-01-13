use clap::Parser;
use mini_geojson::args::Args;
use mini_geojson::file_operations::{
    extract_filename_from_path, process_geojson, read_json_file, write_geojson_file,
};

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

    let mut geojson = read_json_file(&input).unwrap();

    match process_geojson(&mut geojson, decimal) {
        Ok(()) => {
            println!("GeoJSON processed successfully.");
            match write_geojson_file(&geojson, &output) {
                Ok(()) => println!("GeoJSON written successfully."),
                Err(e) => println!("Error writing GeoJSON: {}", e),
            }
        }
        Err(e) => println!("Error processing GeoJSON: {}", e),
    }
}
