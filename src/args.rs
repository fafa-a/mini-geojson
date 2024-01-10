use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Fafa",name = "mini-geojson",  version, about, long_about = None)]
pub struct Args {
    /// Sets the path to the input GeoJSON file
    #[clap(short, long, required = true)]
    pub input: String,

    /// Sets the path to the output GeoJSON file
    #[clap(short, long, default_value = "min_input_filename.geojson")]
    pub output: String,

    /// Sets the number of decimals to truncate to
    #[clap(short, long, required = true)]
    pub decimal: usize,
}
