use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author = "Fafa",name = "mini-geojson",  version, about, long_about = None)]
pub struct Args {
    /// Sets the path to the input GeoJSON file
    #[clap(short, long, required = true)]
    pub input: String,

    /// Sets the path to the output GeoJSON file
    #[clap(short, long, default_value = "./output/")]
    pub output: String,

    /// Sets the number of decimals to keep
    #[clap(short, long, required = true)]
    pub decimal: usize,

    /// Overwrites the output file if it already exists
    #[clap(short = 'O', long, action = ArgAction::SetTrue)]
    pub overwrite: bool,

    /// Pretty write the output file
    #[clap(short, long, action = ArgAction::SetTrue)]
    pub pretty: bool,
}
