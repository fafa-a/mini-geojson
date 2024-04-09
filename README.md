# MINI GEOJSON

Is a simple tool to minify GeoJSON files.  
To learn Rust with a real project, I decided to create this tool.  
So far, it's working fine, but I'm still learning Rust, so I'm sure there are a lot of improvements to be made.

## Features

- Truncate coordinates to a fixed number of decimal places
- Remove properties with null or empty values
- Remove properties with specific names
- Keep only specific properties

## Installation

### From source

You need to have Rust installed on your machine, or you can find the installation instructions [here](https://www.rust-lang.org/tools/install).

```bash
git clone git@github.com:fafa-a/mini-geojson.git 
cd mini-geojson
cargo build --release
```
To run the program, use:
```bash
./target/release/mini-geojson
```

On Windows, use:
```bash
.\target\release\mini-geojson.exe
```

### From release

1. Go to the [releases page](https://github.com/fafa-a/mini-geojson/releases/latest) of the mini-geojson repository.

2. Download the appropriate binary for your operating system (e.g., `mini-geojson-linux` for Linux, `mini-geojson-windows.exe` for Windows).

3. After downloading the file, make it executable.

   On Linux:
   ```bash
   chmod +x /path/to/mini-geojson-linux
   ```
   You can also add the binary to your PATH to run it from anywhere.

   ```bash
   sudo mv path/to/mini-geojson-linux /usr/local/bin/mini-geojson
   ```
    
4. Or you can run the following commands to download the binary and make it executable.
   
   ```bash
   curl -LO https://github.com/fafa-a/mini-geojson/releases/latest/download/mini-geojson-linux
   chmod +x mini-geojson-linux
   sudo mv mini-geojson-linux /usr/local/bin/mini-geojson
   source ~/.zshrc
   ```
    

5. You can now run the program like this:

   On Linux:
   ```bash
   mini-geojson --help
   ```

   On Windows, just double click the `.exe` file to run it.



## Usage

```bash
-i, --input,
  Sets the path to the input GeoJSON file.

-o,--output
  Sets the path to the output GeoJSON file.
  Note: '.json' extension will be replaced with '.geojson'.
  If the output is not specified a "/output/" directory will be created in the same directory as the input file.

  Examples:
  - '/your-output-dir/your-output-file.json'
    will be '/your-output-dir/your-output-file.geojson'.

  - './output/'
    will use the input filename with '.geojson' extension.

-d, --decimal
  Sets the number of decimals to keep.

-O, --overwrite
  Overwrites the output file if it already exists.
  'false' by default.

-p, --pretty
  Pretty write the output file.
  Note: this will increase the file size.
  'false' by default.

-R, --remove-null-properties
  Remove the properties with null values and empty string.
  'false' by default.

-r, --properties-to-remove
  Remove the properties with the specified keys.
  example: -r key1 key2 key3 or -r key1,key2,key3
  Note: -r or -k can be used together.

-k, --properties-to-keep
  Keep only the properties with the specified keys.
  example: -k key1 key2 key3 or -k key1,key2,key3
  Note: -r or -k can be used together.
```


