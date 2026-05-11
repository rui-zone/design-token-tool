use serde_json::{Map, Value};

use super::token::{ensure_token_type, is_alias, is_metadata_key, token};
use super::values::validate_dimension_or_alias;

pub(super) fn convert_typography_group(
    reference: &str,
    group: &str,
    value: &Value,
) -> Result<Value, String> {
    let object = value.as_object().ok_or_else(|| {
        format!("invalid token group `{group}` in `{reference}`: expected object")
    })?;
    let group_type = object.get("$type").and_then(Value::as_str);
    let mut converted = Map::new();

    for (name, token) in object {
        if is_metadata_key(name) {
            continue;
        }

        let token_path = format!("{group}.{name}");
        let token_object = token.as_object().ok_or_else(|| {
            format!("invalid typography token `{token_path}` in `{reference}`: expected object")
        })?;
        let token_type = token_object
            .get("$type")
            .and_then(Value::as_str)
            .or(group_type);
        ensure_token_type(reference, &token_path, token_type, &["typography"])?;

        let value = token_object
            .get("$value")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                format!("missing typography `$value` for `{token_path}` in `{reference}`")
            })?;

        let mut typography = Map::new();
        insert_font_family(reference, &token_path, value, &mut typography)?;
        insert_dimension_field(
            reference,
            &token_path,
            value,
            "fontSize",
            &mut typography,
            true,
        )?;
        insert_line_height(reference, &token_path, value, &mut typography)?;
        insert_dimension_field(
            reference,
            &token_path,
            value,
            "letterSpacing",
            &mut typography,
            false,
        )?;
        insert_font_weight(reference, &token_path, value, &mut typography)?;

        converted.insert(name.clone(), Value::Object(typography));
    }

    Ok(Value::Object(converted))
}

fn insert_font_family(
    reference: &str,
    token_path: &str,
    value: &Map<String, Value>,
    output: &mut Map<String, Value>,
) -> Result<(), String> {
    let Some(field) = value.get("fontFamily") else {
        return Ok(());
    };

    match field {
        Value::String(_) => {
            output.insert("fontFamily".to_string(), token("fontFamily", field.clone()));
            Ok(())
        }
        _ => Err(format!(
            "invalid fontFamily value for `{token_path}` in `{reference}`: expected string or alias"
        )),
    }
}

fn insert_dimension_field(
    reference: &str,
    token_path: &str,
    value: &Map<String, Value>,
    field: &str,
    output: &mut Map<String, Value>,
    required: bool,
) -> Result<(), String> {
    let Some(field_value) = value.get(field) else {
        if required {
            return Err(format!(
                "missing `{field}` for `{token_path}` in `{reference}`"
            ));
        }
        return Ok(());
    };

    validate_dimension_or_alias(reference, &format!("{token_path}.{field}"), field_value)?;
    output.insert(field.to_string(), token("dimension", field_value.clone()));
    Ok(())
}

fn insert_line_height(
    reference: &str,
    token_path: &str,
    value: &Map<String, Value>,
    output: &mut Map<String, Value>,
) -> Result<(), String> {
    let Some(field_value) = value.get("lineHeight") else {
        return Ok(());
    };

    match field_value {
        Value::Number(_) => {
            output.insert(
                "lineHeight".to_string(),
                token("number", field_value.clone()),
            );
            Ok(())
        }
        Value::Object(_) => {
            validate_dimension_or_alias(
                reference,
                &format!("{token_path}.lineHeight"),
                field_value,
            )?;
            output.insert(
                "lineHeight".to_string(),
                token("dimension", field_value.clone()),
            );
            Ok(())
        }
        Value::String(value) if is_alias(value) => Err(format!(
            "unsupported typography alias for `{token_path}.lineHeight` in `{reference}`: aliases must target a Figma-supported scalar token"
        )),
        _ => Err(format!(
            "invalid lineHeight value for `{token_path}` in `{reference}`: expected number or px dimension"
        )),
    }
}

fn insert_font_weight(
    reference: &str,
    token_path: &str,
    value: &Map<String, Value>,
    output: &mut Map<String, Value>,
) -> Result<(), String> {
    let Some(field_value) = value.get("fontWeight") else {
        return Ok(());
    };

    match field_value {
        Value::Number(_) => {
            output.insert(
                "fontWeight".to_string(),
                token("number", field_value.clone()),
            );
            Ok(())
        }
        Value::String(value) if !value.is_empty() => {
            output.insert(
                "fontWeight".to_string(),
                token("string", field_value.clone()),
            );
            Ok(())
        }
        _ => Err(format!(
            "invalid fontWeight value for `{token_path}` in `{reference}`: expected number, string, or alias"
        )),
    }
}
