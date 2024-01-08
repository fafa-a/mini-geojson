use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Fafa",name = "mini-geojson",  version, about, long_about = None)]
struct Args {
    /// Sets the path to the input GeoJSON file
    #[clap(short, long, required = true)]
    input: String,

    /// Sets the path to the output GeoJSON file
    #[clap(short, long, default_value = "min_input_filename.geojson")]
    output: String,

    /// Sets the number of decimals to truncate to
    #[clap(short, long, required = true)]
    decimal: usize,
}

fn main() {
    let args = Args::parse();
    let (input, output, decimal) = (args.input, args.output, args.decimal);
    println!(
        "Input file: {} \nOutput file: {} \nDecimal places to truncate: {}",
        input, output, decimal
    );
}
