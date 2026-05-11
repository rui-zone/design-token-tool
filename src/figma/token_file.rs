use serde_json::{Map, Value};

use super::token::{ensure_token_type, is_metadata_key};
use super::typography::convert_typography_group;
use super::values::validate_token_value;

pub(super) fn merge_token_file(
    reference: &str,
    token_file: &Value,
    root: &mut Map<String, Value>,
) -> Result<(), String> {
    let object = token_file
        .as_object()
        .ok_or_else(|| format!("invalid token source `{reference}`: expected a JSON object"))?;

    for (group, value) in object {
        if is_metadata_key(group) {
            continue;
        }

        let converted = match group.as_str() {
            "colors" => convert_regular_group(reference, group, value, &["color"])?,
            "spacing" => convert_regular_group(reference, group, value, &["dimension", "number"])?,
            "radius" => convert_regular_group(reference, group, value, &["dimension", "number"])?,
            "typography" => convert_typography_group(reference, group, value)?,
            _ => {
                return Err(format!(
                    "unsupported token group `{group}` in token source `{reference}`"
                ));
            }
        };

        merge_group(root, group, converted)?;
    }

    Ok(())
}

fn merge_group(root: &mut Map<String, Value>, group: &str, value: Value) -> Result<(), String> {
    match (root.get_mut(group), value) {
        (None, value) => {
            root.insert(group.to_string(), value);
        }
        (Some(Value::Object(existing)), Value::Object(incoming)) => {
            for (key, value) in incoming {
                existing.insert(key, value);
            }
        }
        (Some(_), _) => {
            return Err(format!(
                "invalid merged token group `{group}`: expected JSON objects"
            ));
        }
    }

    Ok(())
}

fn convert_regular_group(
    reference: &str,
    group: &str,
    value: &Value,
    expected_types: &[&str],
) -> Result<Value, String> {
    convert_regular_node(reference, group, value, None, expected_types)
}

fn convert_regular_node(
    reference: &str,
    token_path: &str,
    value: &Value,
    inherited_type: Option<&str>,
    expected_types: &[&str],
) -> Result<Value, String> {
    let object = value.as_object().ok_or_else(|| {
        format!("invalid token group `{token_path}` in `{reference}`: expected object")
    })?;
    let node_type = object
        .get("$type")
        .and_then(Value::as_str)
        .or(inherited_type);

    if object.contains_key("$value") {
        ensure_token_type(reference, token_path, node_type, expected_types)?;
        validate_token_value(reference, token_path, node_type, value)?;
        return Ok(value.clone());
    }

    let mut converted = Map::new();
    for (key, child) in object {
        if is_metadata_key(key) {
            converted.insert(key.clone(), child.clone());
            continue;
        }

        converted.insert(
            key.clone(),
            convert_regular_node(
                reference,
                &format!("{token_path}.{key}"),
                child,
                node_type,
                expected_types,
            )?,
        );
    }

    Ok(Value::Object(converted))
}
