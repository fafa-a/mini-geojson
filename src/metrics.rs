use std::{fs, path::Path};

use size::Size;

fn get_file_size(file_path: &str) -> std::io::Result<u64> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len())
}

fn calculate_size_reduction(original_size: u64, minified_size: u64) -> f64 {
    let reduction = original_size as f64 - minified_size as f64;
    (reduction / original_size as f64) * 100.0
}

pub fn calculate_and_display_size_reduction(input_path: &str, output_path: &Path) {
    let original_size = get_file_size(input_path).unwrap();
    let minified_size = get_file_size(&output_path.to_string_lossy()).unwrap();
    let original_size_formatted = Size::from_bytes(original_size).to_string();
    let minified_size_formatted = Size::from_bytes(minified_size).to_string();

    let reduction_percentage = calculate_size_reduction(original_size, minified_size);
    println!(
        "File size reduced by {:.2}% (from {} to {})",
        reduction_percentage, original_size_formatted, minified_size_formatted
    );
}
