use serde_json::Value;

use super::css::CssVariables;
use super::names::{NameMode, css_name_suffix_for_token};
use super::token::{ensure_token_type, token_entries, token_type};
use super::typography_fields::{
    insert_letter_spacing, insert_line_height, insert_required_font_size, insert_string,
    typography_value,
};
use super::typography_validate::validate_fields;
use super::typography_weight::insert_font_weight;

pub(super) fn collect_typography_variables(
    reference: &str,
    group: &str,
    value: &Value,
    variables: &mut CssVariables,
) -> Result<(), String> {
    for token in token_entries(reference, group, value)? {
        let token_type = token_type(token.value, token.group_type);
        ensure_token_type(reference, &token.path, token_type, &["typography"])?;

        let token_name =
            css_name_suffix_for_token(reference, &token.path, &token.name, NameMode::Typography)?;
        let value = typography_value(reference, &token.path, token.value)?;

        insert_string(
            value,
            "fontFamily",
            reference,
            &token.path,
            variables,
            || format!("--font-{token_name}"),
        )?;
        insert_required_font_size(value, reference, &token.path, &token_name, variables)?;
        insert_line_height(value, reference, &token.path, &token_name, variables)?;
        insert_letter_spacing(value, reference, &token.path, &token_name, variables)?;
        insert_font_weight(value, reference, &token.path, &token_name, variables)?;
        insert_string(
            value,
            "fontFeature",
            reference,
            &token.path,
            variables,
            || format!("--font-{token_name}--font-feature-settings"),
        )?;
        insert_string(
            value,
            "fontVariation",
            reference,
            &token.path,
            variables,
            || format!("--font-{token_name}--font-variation-settings"),
        )?;
        validate_fields(reference, &token.path, value)?;
    }

    Ok(())
}
