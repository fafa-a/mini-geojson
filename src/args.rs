use clap::{builder::ValueRange, ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author = "Fafa",name = "mini-geojson",  version, about, long_about = None)]
pub struct Args {
    /// Sets the path to the input GeoJSON file.
    #[clap(short, long, required = true)]
    pub input: String,

    /// Sets the path to the output GeoJSON file.
    /// Note: '.json' extension will be replaced with '.geojson'.
    ///
    /// Examples:
    /// - '/your-output-dir/your-output-file.json'
    ///   will be '/your-output-dir/your-output-file.geojson'.
    ///
    /// - './output/'
    ///   will use the input filename with '.geojson' extension.
    #[clap(short, long, default_value = "./output/")]
    pub output: String,

    /// Sets the number of decimals to keep.
    #[clap(short, long)]
    pub decimal: Option<usize>,

    /// Overwrites the output file if it already exists.
    #[clap(short = 'O', long, action = ArgAction::SetTrue)]
    pub overwrite: bool,

    /// Pretty write the output file.
    /// Note: this will increase the file size.
    #[clap(short, long, action = ArgAction::SetTrue)]
    pub pretty: bool,

    /// Remove the properties with null values and empty string.
    #[clap(short = 'R', long, action = ArgAction::SetTrue)]
    pub remove_null_properties: bool,

    /// Remove the properties with the specified keys.
    /// example: -r key1 key2 key3 or -r key1,key2,key3
    /// Note: -r or -k can be used together.
    #[clap(short = 'r', long, num_args = ValueRange::new(0..), value_delimiter = ',')]
    pub properties_to_remove: Option<Vec<String>>,

    /// Keep only the properties with the specified keys.
    /// example: -k key1 key2 key3 or -k key1,key2,key3
    /// Note: -r or -k can be used together.
    #[clap(short = 'k', long, num_args = ValueRange::new(0..), value_delimiter = ',')]
    pub properties_to_keep: Option<Vec<String>>,
}
