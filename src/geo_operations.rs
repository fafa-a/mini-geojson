use log::{debug, info};
use sonic_rs::{JsonValueMutTrait, JsonValueTrait, Value};

pub fn truncate_coordinate_in_array(coordinates: &mut Value, decimal: usize) {
    info!(
        "Starting coordinate truncation with decimal precision: {}",
        decimal
    );

    fn truncate_recursive(coord: &mut Value, decimal: usize) {
        debug!("Truncating coordinate: {:?}", coord);
        if coord.is_array() {
            if let Some(coords_array) = coord.as_array_mut() {
                for c in coords_array {
                    truncate_recursive(c, decimal);
                }
            }
        } else if coord.is_number() {
            if let Some(number) = coord.as_f64() {
                let truncated_number = truncate_coord(number, decimal);
                debug!(
                    "Original number: {}, Truncated number: {}",
                    number, truncated_number
                );
                *coord = truncated_number.try_into().unwrap()
            }
        }
    }
    truncate_recursive(coordinates, decimal);
    info!("Coordinate truncation completed");
}

pub fn truncate_coord(coord: f64, decimal: usize) -> f64 {
    let multiplier = 10u64.pow(decimal as u32) as f64;
    (coord * multiplier).round() / multiplier
}

pub fn process_feature(
    feature: &mut Value,
    decimal: Option<usize>,
    remove_null_properties: bool,
    properties_to_remove: Option<&Vec<String>>,
    properties_to_keep: Option<&Vec<String>>,
) {
    if let Some(geometry) = feature.get_mut("geometry").and_then(|g| g.as_object_mut()) {
        if let Some(coords) = geometry.get_mut(&"coordinates".to_string()) {
            if let Some(decimal_value) = decimal {
                truncate_coordinate_in_array(coords, decimal_value);
            }
        }
        if remove_null_properties {
            remove_null_or_empty_properties(feature);
        }

        if let Some(keys) = properties_to_remove {
            remove_properties(feature, keys);
        }

        if let Some(keys) = properties_to_keep {
            keep_properties(feature, keys);
        }
    }
}


fn remove_null_or_empty_properties(geojson: &mut Value) {
    debug!("Removing null or empty properties");
    println!("Removing null or empty properties");
    if let Some(properties) = geojson
        .get_mut("properties")
        .and_then(|p| p.as_object_mut())
    {
        let keys_to_remove: Vec<String> = properties
            .iter()
            .filter_map(|(key, value)| {
                if value.is_null() || value.as_str() == Some("") {
                    Some(key.to_string())
                } else {
                    None
                }
            })
            .collect();
        for key in keys_to_remove {
            debug!("Removing property: {}", key);
            println!("Removing property: {}", key);
            properties.remove(&key);
        }
    }
}

fn remove_properties(geojson: &mut Value, keys: &Vec<String>) {
    debug!("Removing properties: {:?}", keys);
    println!("Removing properties: {:?}", keys);
    if let Some(properties) = geojson
        .get_mut("properties")
        .and_then(|p| p.as_object_mut())
    {
        for key in keys {
            println!("Removing property: {}", key);
            debug!("Removing property: {}", key);
            properties.remove(&key);
        }
    }
}

fn keep_properties(geojson: &mut Value, keys: &Vec<String>) {
    debug!("Keeping properties: {:?}", keys);
    println!("Keeping properties: {:?}", keys);
    if let Some(properties) = geojson
        .get_mut("properties")
        .and_then(|p| p.as_object_mut())
    {
        let keys_to_remove: Vec<String> = properties
            .iter()
            .filter_map(|(key, _)| {
                if !keys.contains(&key.to_string()){
                    Some(key.to_string())
                } else {
                    None
                }
            })
            .collect();
        for key in keys_to_remove {
            println!("Removing property: {}", key);
            debug!("Removing property: {}", key);
            properties.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use sonic_rs::json;

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

    #[test]
    fn test_remove_null_or_empty_properties() {
        let mut geojson = json!({
                "type": "Feature",
                "properties": {
                    "name": "test",
                    "empty": "",
                    "null": null,
                    "value": "value"
        },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.234567, 2.345678]
                }
            });

        remove_null_or_empty_properties(&mut geojson);

        assert_eq!(
            geojson,
            json!({
                "type": "Feature",
                "properties": {
                    "name": "test",
                    "value": "value"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.234567, 2.345678]
                }
            })
        );
    }

    #[test]
    fn test_process_feature_with_only_truncation() {
        let mut geojson = json!({
            "type": "Feature",
            "properties": {
                "name": "test",
                "empty": "",
                "null": null,
                "value": "value"
            },
            "geometry": {
                "type": "Point",
                "coordinates": [1.234567, 2.345678]
            }
        });

        process_feature(&mut geojson, Some(2), false, None, None);

        assert_eq!(
            geojson,
            json!({
                "type": "Feature",
                "properties": {
                    "name": "test",
                    "empty": "",
                    "null": null,
                    "value": "value"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.23, 2.35]
                }
            })
        );
    }

    #[test]
    fn test_process_feature_with_only_remove_null_properties() {
        let mut geojson = json!({
            "type": "Feature",
            "properties": {
                "name": "test",
                "empty": "",
                "null": null,
                "value": "value"
            },
            "geometry": {
                "type": "Point",
                "coordinates": [1.234567, 2.345678]
            }
        });

        process_feature(&mut geojson, None, true, None, None);

        assert_eq!(
            geojson,
            json!({
                "type": "Feature",
                "properties": {
                    "name": "test",
                    "value": "value"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.234567, 2.345678]
                }
            })
        );
    }

    #[test]
    fn test_process_feature_with_truncation_and_remove_null_properties() {
        let mut geojson = json!({
            "type": "Feature",
            "properties": {
                "name": "test",
                "empty": "",
                "null": null,
                "value": "value"
            },
            "geometry": {
                "type": "Point",
                "coordinates": [1.234567, 2.345678]
            }
        });

        process_feature(&mut geojson, Some(2), true, None, None);

        assert_eq!(
            geojson,
            json!({
                "type": "Feature",
                "properties": {
                    "name": "test",
                    "value": "value"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.23, 2.35]
                }
            })
        );
    }

    #[test]
    fn test_process_feature_with_only_remove_properties() {
        let mut geojson = json!({
            "type": "Feature",
            "properties": {
                "name": "test",
                "empty": "",
                "null": null,
                "value": "value"
            },
            "geometry": {
                "type": "Point",
                "coordinates": [1.234567, 2.345678]
            }
        });

        process_feature(
            &mut geojson,
            None,
            false,
            Some(&vec!["name".to_string(), "empty".to_string()]),
            None,
        );

        assert_eq!(
            geojson,
            json!({
                "type": "Feature",
                "properties": {
                    "null": null,
                    "value": "value"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.234567, 2.345678]
                }
            })
        );
    }

    #[test]
    fn test_process_feature_with_only_keep_properties() {
        let mut geojson = json!({
            "type": "Feature",
            "properties": {
                "name": "test",
                "empty": "",
                "null": null,
                "value": "value"
            },
            "geometry": {
                "type": "Point",
                "coordinates": [1.234567, 2.345678]
            }
        });

        process_feature(
            &mut geojson,
            None,
            false,
            None,
            Some(&vec!["name".to_string(), "empty".to_string()]),
        );

        assert_eq!(
            geojson,
            json!({
                "type": "Feature",
                "properties": {
                    "name": "test",
                    "empty": ""
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.234567, 2.345678]
                }
            })
        );
    }

    #[test]
    fn test_process_feature_with_truncation_and_keep_properties() {
        let mut geojson = json!({
            "type": "Feature",
            "properties": {
                "name": "test",
                "empty": "",
                "null": null,
                "value": "value"
            },
            "geometry": {
                "type": "Point",
                "coordinates": [1.234567, 2.345678]
            }
        });

        process_feature(
            &mut geojson,
            Some(2),
            false,
            None,
            Some(&vec!["name".to_string(), "empty".to_string()]),
        );

        assert_eq!(
            geojson,
            json!({
                "type": "Feature",
                "properties": {
                    "name": "test",
                    "empty": ""
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [1.23, 2.35]
                }
            })
        );
    }
}
