use serde_json::{Map, Value};

pub(super) fn dimension_css_value(
    reference: &str,
    token_path: &str,
    dimension: &Map<String, Value>,
) -> Result<String, String> {
    let value = dimension
        .get("value")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            format!("invalid dimension value for `{token_path}` in `{reference}`: missing value")
        })?;
    let unit = dimension
        .get("unit")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!("invalid dimension value for `{token_path}` in `{reference}`: missing unit")
        })?;

    if unit != "px" && unit != "rem" {
        return Err(format!(
            "invalid dimension value for `{token_path}` in `{reference}`: expected px or rem"
        ));
    }

    Ok(format!("{}{}", format_number(value), unit))
}

pub(super) fn validate_css_value(
    reference: &str,
    token_path: &str,
    value: &str,
) -> Result<String, String> {
    if value.is_empty()
        || value
            .chars()
            .any(|character| matches!(character, ';' | '{' | '}' | '\n' | '\r'))
    {
        return Err(format!(
            "invalid CSS value for `{token_path}` in `{reference}`"
        ));
    }

    Ok(value.to_string())
}

pub(super) fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        return format!("{value:.0}");
    }

    value.to_string()
}
