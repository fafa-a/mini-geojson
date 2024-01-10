mod file_operations;

use clap::Parser;
use mini_geojson::args::Args;
use mini_geojson::file_operations::extract_filename_from_path;

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
}
