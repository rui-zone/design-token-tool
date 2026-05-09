use serde_json::{Map, Value};

use super::css::CssVariables;
use super::values::font_weight_css_value;

pub(super) fn insert_font_weight(
    value: &Map<String, Value>,
    reference: &str,
    token_path: &str,
    token_name: &str,
    variables: &mut CssVariables,
) -> Result<(), String> {
    if let Some(raw) = value.get("fontWeight") {
        variables.insert(
            format!("--text-{token_name}--font-weight"),
            font_weight_css_value(reference, &format!("{token_path}.fontWeight"), raw)?,
        );
    }

    Ok(())
}
