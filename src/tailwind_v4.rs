mod alias;
mod alias_typography;
mod color;
mod css;
mod names;
mod radius;
mod resolver;
mod spacing;
#[cfg(test)]
mod tests;
mod token;
mod token_file;
mod typography;
mod typography_fields;
mod typography_validate;
mod typography_weight;
mod value_format;
mod values;

use serde_json::Value;

use css::{CssVariables, render_css};
use resolver::collect_resolver_refs;
use token_file::{collect_token_file_variables, parse_token_source};

pub const TAILWIND_V4_THEME_FILE: &str = "theme.css";

/// Converts a DTCG resolver and its referenced token files into Tailwind CSS v4 theme variables.
pub fn convert_resolver_to_tailwind_v4<F>(
    resolver_source: &str,
    mut load_ref: F,
) -> Result<String, String>
where
    F: FnMut(&str) -> Result<String, String>,
{
    let resolver: Value = serde_json::from_str(resolver_source)
        .map_err(|error| format!("failed to parse tokens resolver JSON: {error}"))?;
    let refs = collect_resolver_refs(&resolver)?;

    let mut base = CssVariables::default();
    let mut dark = CssVariables::default();

    for reference in refs.foundation.iter().chain(refs.light.iter()) {
        let token_file = load_token_file(reference, &mut load_ref)?;
        collect_token_file_variables(reference, &token_file, &mut base)?;
    }

    for reference in refs.dark {
        let token_file = load_token_file(&reference, &mut load_ref)?;
        collect_token_file_variables(&reference, &token_file, &mut dark)?;
    }

    Ok(render_css(&base, &dark))
}

fn load_token_file<F>(reference: &str, load_ref: &mut F) -> Result<Value, String>
where
    F: FnMut(&str) -> Result<String, String>,
{
    let source = load_ref(reference)
        .map_err(|error| format!("failed to load resolver $ref `{reference}`: {error}"))?;
    parse_token_source(reference, &source)
}
