use serde_json::Value;

use super::alias::{alias_to_css_var, is_alias};
use super::value_format::{dimension_css_value, validate_css_value};

pub(super) fn dimension_or_alias_css_value(
    reference: &str,
    token_path: &str,
    value: &Value,
) -> Result<String, String> {
    match value {
        Value::String(value) if is_alias(value) => alias_to_css_var(reference, token_path, value),
        Value::Object(dimension) => dimension_css_value(reference, token_path, dimension),
        _ => Err(format!(
            "invalid dimension value for `{token_path}` in `{reference}`: expected DTCG dimension object or alias"
        )),
    }
}

pub(super) fn number_or_alias_css_value(
    reference: &str,
    token_path: &str,
    value: &Value,
) -> Result<String, String> {
    match value {
        Value::String(value) if is_alias(value) => alias_to_css_var(reference, token_path, value),
        Value::Number(number) => Ok(number.to_string()),
        _ => Err(format!(
            "invalid number value for `{token_path}` in `{reference}`: expected number or alias"
        )),
    }
}

pub(super) fn string_or_alias_css_value(
    reference: &str,
    token_path: &str,
    value: &Value,
) -> Result<String, String> {
    match value {
        Value::String(value) if is_alias(value) => alias_to_css_var(reference, token_path, value),
        Value::String(value) => validate_css_value(reference, token_path, value),
        _ => Err(format!(
            "invalid string value for `{token_path}` in `{reference}`: expected string or alias"
        )),
    }
}

pub(super) fn line_height_css_value(
    reference: &str,
    token_path: &str,
    value: &Value,
) -> Result<String, String> {
    match value {
        Value::String(value) if is_alias(value) => alias_to_css_var(reference, token_path, value),
        Value::Number(number) => Ok(number.to_string()),
        Value::Object(dimension) => dimension_css_value(reference, token_path, dimension),
        _ => Err(format!(
            "invalid lineHeight value for `{token_path}` in `{reference}`: expected number, dimension, or alias"
        )),
    }
}

pub(super) fn font_weight_css_value(
    reference: &str,
    token_path: &str,
    value: &Value,
) -> Result<String, String> {
    match value {
        Value::String(value) if is_alias(value) => alias_to_css_var(reference, token_path, value),
        Value::String(value) => validate_css_value(reference, token_path, value),
        Value::Number(number) if number.as_i64() == Some(400) => Ok("normal".to_string()),
        Value::Number(number) if number.as_i64() == Some(700) => Ok("bold".to_string()),
        Value::Number(number) => Ok(number.to_string()),
        _ => Err(format!(
            "invalid fontWeight value for `{token_path}` in `{reference}`: expected number, string, or alias"
        )),
    }
}
