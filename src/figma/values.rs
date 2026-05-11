use serde_json::Value;

use super::token::is_alias;

pub(super) fn validate_token_value(
    reference: &str,
    token_path: &str,
    token_type: Option<&str>,
    token: &Value,
) -> Result<(), String> {
    let value = token
        .get("$value")
        .ok_or_else(|| format!("missing `$value` for `{token_path}` in `{reference}`"))?;

    match token_type {
        Some("color") => validate_color_or_alias(reference, token_path, value),
        Some("dimension") => validate_dimension_or_alias(reference, token_path, value),
        Some("number") => validate_number_or_alias(reference, token_path, value),
        Some(other) => Err(format!(
            "unsupported token type for `{token_path}` in `{reference}`: {other}"
        )),
        None => Err(format!(
            "unsupported token type for `{token_path}` in `{reference}`: expected a Figma-supported token type, got <missing>"
        )),
    }
}

pub(super) fn validate_dimension_or_alias(
    reference: &str,
    token_path: &str,
    value: &Value,
) -> Result<(), String> {
    match value {
        Value::String(value) if is_alias(value) => Ok(()),
        Value::Object(dimension) => {
            let unit = dimension
                .get("unit")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    format!(
                        "invalid dimension value for `{token_path}` in `{reference}`: missing unit"
                    )
                })?;
            let has_number = dimension.get("value").and_then(Value::as_f64).is_some();
            if !has_number {
                return Err(format!(
                    "invalid dimension value for `{token_path}` in `{reference}`: missing value"
                ));
            }
            if unit != "px" {
                return Err(format!(
                    "invalid dimension value for `{token_path}` in `{reference}`: Figma import supports px only, got {unit}"
                ));
            }
            Ok(())
        }
        _ => Err(format!(
            "invalid dimension value for `{token_path}` in `{reference}`: expected px dimension object or alias"
        )),
    }
}

fn validate_color_or_alias(reference: &str, token_path: &str, value: &Value) -> Result<(), String> {
    match value {
        Value::String(value) if is_alias(value) => Ok(()),
        Value::Object(color) => {
            let color_space = color
                .get("colorSpace")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    format!(
                        "invalid color value for `{token_path}` in `{reference}`: missing colorSpace"
                    )
                })?;
            if color_space != "srgb" && color_space != "hsl" {
                return Err(format!(
                    "invalid color value for `{token_path}` in `{reference}`: expected srgb or hsl"
                ));
            }
            Ok(())
        }
        _ => Err(format!(
            "invalid color value for `{token_path}` in `{reference}`: expected DTCG color object or alias"
        )),
    }
}

fn validate_number_or_alias(
    reference: &str,
    token_path: &str,
    value: &Value,
) -> Result<(), String> {
    match value {
        Value::String(value) if is_alias(value) => Ok(()),
        Value::Number(_) => Ok(()),
        _ => Err(format!(
            "invalid number value for `{token_path}` in `{reference}`: expected number or alias"
        )),
    }
}
