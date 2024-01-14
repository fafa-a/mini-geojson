use log::{debug, info};
use serde_json::{json, Value};

pub fn truncate_coordinate_in_array(coordinates: &mut Value, decimal: usize) {
    info!(
        "Starting coordinate truncation with decimal precision: {}",
        decimal
    );

    fn truncate_recursive(coord: &mut Value, decimal: usize) {
        match coord {
            Value::Array(coords) => {
                for c in coords {
                    truncate_recursive(c, decimal);
                }
            }
            Value::Number(number) => {
                if let Some(number) = number.as_f64() {
                    let truncated_number = truncate_coord(number, decimal);
                    debug!("Truncating coordinate: {} -> {}", number, truncated_number);
                    *coord = json!(truncated_number);
                }
            }
            _ => (),
        }
    }

    truncate_recursive(coordinates, decimal);
    info!("Coordinate truncation completed");
}

pub fn truncate_coord(coord: f64, decimal: usize) -> f64 {
    let multiplier = 10u64.pow(decimal as u32) as f64;
    (coord * multiplier).round() / multiplier
}
