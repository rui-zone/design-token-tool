use serde_json::{Map, Value};

use super::css::CssVariables;
use super::token::token_value;
use super::values::{
    dimension_or_alias_css_value, line_height_css_value, string_or_alias_css_value,
};

pub(super) fn typography_value<'a>(
    reference: &str,
    token_path: &str,
    token: &'a Value,
) -> Result<&'a Map<String, Value>, String> {
    token_value(reference, token_path, token)?
        .as_object()
        .ok_or_else(|| {
            format!("invalid token `{token_path}` in `{reference}`: expected typography object")
        })
}

pub(super) fn insert_string<F>(
    value: &Map<String, Value>,
    field: &str,
    reference: &str,
    token_path: &str,
    variables: &mut CssVariables,
    name: F,
) -> Result<(), String>
where
    F: FnOnce() -> String,
{
    if let Some(raw) = value.get(field) {
        variables.insert(
            name(),
            string_or_alias_css_value(reference, &format!("{token_path}.{field}"), raw)?,
        );
    }
    Ok(())
}

pub(super) fn insert_required_font_size(
    value: &Map<String, Value>,
    reference: &str,
    token_path: &str,
    token_name: &str,
    variables: &mut CssVariables,
) -> Result<(), String> {
    let raw = value.get("fontSize").ok_or_else(|| {
        format!("missing typography field `fontSize` for token `{token_path}` in `{reference}`")
    })?;
    variables.insert(
        format!("--text-{token_name}"),
        dimension_or_alias_css_value(reference, &format!("{token_path}.fontSize"), raw)?,
    );
    Ok(())
}

pub(super) fn insert_line_height(
    value: &Map<String, Value>,
    reference: &str,
    token_path: &str,
    token_name: &str,
    variables: &mut CssVariables,
) -> Result<(), String> {
    if let Some(raw) = value.get("lineHeight") {
        variables.insert(
            format!("--text-{token_name}--line-height"),
            line_height_css_value(reference, &format!("{token_path}.lineHeight"), raw)?,
        );
    }
    Ok(())
}

pub(super) fn insert_letter_spacing(
    value: &Map<String, Value>,
    reference: &str,
    token_path: &str,
    token_name: &str,
    variables: &mut CssVariables,
) -> Result<(), String> {
    if let Some(raw) = value.get("letterSpacing") {
        variables.insert(
            format!("--text-{token_name}--letter-spacing"),
            dimension_or_alias_css_value(reference, &format!("{token_path}.letterSpacing"), raw)?,
        );
    }
    Ok(())
}
