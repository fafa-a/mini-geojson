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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_truncate_coord() {
        let mut coordinates = json!([1.234567, 2.345678]);
        let decimal = 2;

        truncate_coordinate_in_array(&mut coordinates, decimal);

        assert_eq!(coordinates, json!([1.23, 2.35]));
    }

    #[test]
    fn test_truncate_coordinate_in_array_of_array() {
        let mut coordinates = json!([[1.234567, 2.345678], [4.567890, 5.678901]]);
        let decimal = 2;

        truncate_coordinate_in_array(&mut coordinates, decimal);

        assert_eq!(coordinates, json!([[1.23, 2.35], [4.57, 5.68]]));
    }

    #[test]
    fn truncate_coordinate_in_geojson_geometry() {
        let mut coordinates = json!([
            [
                [1.234567, 2.345678],
                [4.567890, 5.678901],
                [7.890123, 8.901234],
                [1.234567, 2.345678]
            ],
            [
                [1.234567, 2.345678],
                [4.567890, 5.678901],
                [7.890123, 8.901234],
                [1.234567, 2.345678]
            ],
            [
                [1.234567, 2.345678],
                [4.567890, 5.678901],
                [7.890123, 8.901234],
                [1.234567, 2.345678]
            ]
        ]);
        let decimal = 2;
        truncate_coordinate_in_array(&mut coordinates, decimal);
        assert_eq!(
            coordinates,
            json!([
                [[1.23, 2.35], [4.57, 5.68], [7.89, 8.90], [1.23, 2.35]],
                [[1.23, 2.35], [4.57, 5.68], [7.89, 8.90], [1.23, 2.35]],
                [[1.23, 2.35], [4.57, 5.68], [7.89, 8.90], [1.23, 2.35]]
            ])
        );
    }
}
