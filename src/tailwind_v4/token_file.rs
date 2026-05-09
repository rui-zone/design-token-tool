use serde_json::Value;

use super::color::collect_color_variables;
use super::css::CssVariables;
use super::radius::collect_radius_variables;
use super::spacing::collect_spacing_variables;
use super::token::is_metadata_key;
use super::typography::collect_typography_variables;

pub(super) fn parse_token_source(reference: &str, source: &str) -> Result<Value, String> {
    serde_json::from_str(source)
        .map_err(|error| format!("failed to parse token source `{reference}`: {error}"))
}

pub(super) fn collect_token_file_variables(
    reference: &str,
    token_file: &Value,
    variables: &mut CssVariables,
) -> Result<(), String> {
    let root = token_file
        .as_object()
        .ok_or_else(|| format!("invalid token source `{reference}`: expected a JSON object"))?;

    for (group, value) in root {
        if is_metadata_key(group) {
            continue;
        }

        match group.as_str() {
            "colors" => collect_color_variables(reference, group, value, variables)?,
            "spacing" => collect_spacing_variables(reference, group, value, variables)?,
            "radius" => collect_radius_variables(reference, group, value, variables)?,
            "typography" => collect_typography_variables(reference, group, value, variables)?,
            _ => {
                return Err(format!(
                    "unsupported token group `{group}` in token source `{reference}`"
                ));
            }
        }
    }

    Ok(())
}
