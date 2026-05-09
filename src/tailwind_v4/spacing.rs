use serde_json::Value;

use super::css::CssVariables;
use super::names::{NameMode, css_name_suffix_for_token};
use super::token::{ensure_token_type, token_entries, token_type, token_value};
use super::values::{dimension_or_alias_css_value, number_or_alias_css_value};

pub(super) fn collect_spacing_variables(
    reference: &str,
    group: &str,
    value: &Value,
    variables: &mut CssVariables,
) -> Result<(), String> {
    for token in token_entries(reference, group, value)? {
        let token_type = token_type(token.value, token.group_type);
        ensure_token_type(reference, &token.path, token_type, &["dimension", "number"])?;

        let css_name = format!(
            "--spacing-{}",
            css_name_suffix_for_token(reference, &token.path, &token.name, NameMode::Preserve)?
        );
        let token_value = token_value(reference, &token.path, token.value)?;
        let css_value = match token_type {
            Some("dimension") => dimension_or_alias_css_value(reference, &token.path, token_value)?,
            Some("number") => number_or_alias_css_value(reference, &token.path, token_value)?,
            _ => unreachable!("spacing token type should be validated"),
        };
        variables.insert(css_name, css_value);
    }

    Ok(())
}
