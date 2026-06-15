use serde_json::{Map, Value};

use super::alias::{alias_to_css_var, is_alias};
use super::css::CssVariables;
use super::names::{NameMode, css_name_suffix_for_token};
use super::token::{ensure_token_type, token_entries, token_type, token_value};
use super::value_format::format_number;

pub(super) fn collect_color_variables(
    reference: &str,
    group: &str,
    value: &Value,
    variables: &mut CssVariables,
) -> Result<(), String> {
    for token in token_entries(reference, group, value)? {
        let token_type = token_type(token.value, token.group_type);
        ensure_token_type(reference, &token.path, token_type, &["color"])?;

        let css_name = format!(
            "--color-{}",
            css_name_suffix_for_token(reference, &token.path, &token.name, NameMode::Preserve)?
        );
        let css_value = color_css_value(
            reference,
            &token.path,
            token_value(reference, &token.path, token.value)?,
        )?;
        variables.insert(css_name, css_value);
    }

    Ok(())
}

fn color_css_value(reference: &str, token_path: &str, value: &Value) -> Result<String, String> {
    match value {
        Value::String(value) if is_alias(value) => alias_to_css_var(reference, token_path, value),
        Value::String(value) if value.starts_with('#') => Ok(value.to_ascii_lowercase()),
        Value::Object(color) => color_object_css_value(reference, token_path, color),
        _ => Err(format!(
            "invalid color value for `{token_path}` in `{reference}`: expected DTCG color object or alias"
        )),
    }
}

fn color_object_css_value(
    reference: &str,
    token_path: &str,
    color: &Map<String, Value>,
) -> Result<String, String> {
    let color_space = color
        .get("colorSpace")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!("invalid color value for `{token_path}` in `{reference}`: missing colorSpace")
        })?;

    match color_space {
        "srgb" => srgb_color_object_css_value(reference, token_path, color),
        "oklch" => oklch_color_object_css_value(reference, token_path, color),
        _ => Err(format!(
            "unsupported color space `{color_space}` for `{token_path}` in `{reference}`"
        )),
    }
}

fn srgb_color_object_css_value(
    reference: &str,
    token_path: &str,
    color: &Map<String, Value>,
) -> Result<String, String> {
    let alpha = color.get("alpha").and_then(Value::as_f64).ok_or_else(|| {
        format!("invalid color value for `{token_path}` in `{reference}`: missing alpha")
    })?;

    if alpha >= 1.0 {
        let hex = color.get("hex").and_then(Value::as_str).ok_or_else(|| {
            format!("invalid color value for `{token_path}` in `{reference}`: missing hex")
        })?;
        return Ok(hex.to_ascii_lowercase());
    }

    let components = color
        .get("components")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!("invalid color value for `{token_path}` in `{reference}`: missing components")
        })?;
    if components.len() != 3 {
        return Err(format!(
            "invalid color value for `{token_path}` in `{reference}`: expected 3 components"
        ));
    }

    let mut channels = Vec::new();
    for component in components {
        let value = component.as_f64().ok_or_else(|| {
            format!(
                "invalid color value for `{token_path}` in `{reference}`: component is not a number"
            )
        })?;
        channels.push(format_number((value * 255.0).round()));
    }

    Ok(format!(
        "rgb({} {} {} / {})",
        channels[0],
        channels[1],
        channels[2],
        format_number(alpha)
    ))
}

fn oklch_color_object_css_value(
    reference: &str,
    token_path: &str,
    color: &Map<String, Value>,
) -> Result<String, String> {
    let alpha = color.get("alpha").and_then(Value::as_f64).ok_or_else(|| {
        format!("invalid color value for `{token_path}` in `{reference}`: missing alpha")
    })?;

    let components = color
        .get("components")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!("invalid color value for `{token_path}` in `{reference}`: missing components")
        })?;
    if components.len() != 3 {
        return Err(format!(
            "invalid color value for `{token_path}` in `{reference}`: expected 3 components"
        ));
    }

    let mut values = Vec::new();
    for component in components {
        let value = component.as_f64().ok_or_else(|| {
            format!(
                "invalid color value for `{token_path}` in `{reference}`: component is not a number"
            )
        })?;
        values.push(value);
    }

    let lightness = values[0] * 100.0;
    let chroma = values[1];
    let hue = values[2];

    if alpha >= 1.0 {
        Ok(format!(
            "oklch({}% {} {}deg)",
            format_number(lightness),
            format_number(chroma),
            format_number(hue)
        ))
    } else {
        Ok(format!(
            "oklch({}% {} {}deg / {})",
            format_number(lightness),
            format_number(chroma),
            format_number(hue),
            format_number(alpha)
        ))
    }
}
