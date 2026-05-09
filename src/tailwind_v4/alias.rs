use super::alias_typography::typography_alias_css_var;
use super::names::{NameMode, css_name_suffix_for_token};

pub(super) fn alias_to_css_var(
    reference: &str,
    token_path: &str,
    alias: &str,
) -> Result<String, String> {
    let alias_reference = alias_reference(alias).ok_or_else(|| {
        format!("invalid alias for `{token_path}` in `{reference}`: expected {{path.to.token}}")
    })?;
    let segments = alias_reference.split('.').collect::<Vec<_>>();

    let Some(namespace) = segments.first().copied() else {
        return Err(format!(
            "invalid alias for `{token_path}` in `{reference}`: expected namespace"
        ));
    };

    let css_var = match namespace {
        "colors" => format!(
            "--color-{}",
            alias_suffix(
                reference,
                token_path,
                namespace,
                &segments[1..],
                NameMode::Preserve
            )?
        ),
        "spacing" => format!(
            "--spacing-{}",
            alias_suffix(
                reference,
                token_path,
                namespace,
                &segments[1..],
                NameMode::Preserve
            )?
        ),
        "radius" => format!(
            "--radius-{}",
            alias_suffix(
                reference,
                token_path,
                namespace,
                &segments[1..],
                NameMode::Preserve
            )?
        ),
        "typography" => typography_alias_css_var(reference, token_path, namespace, &segments[1..])?,
        _ => {
            return Err(format!(
                "unsupported alias namespace `{namespace}` for `{token_path}` in `{reference}`"
            ));
        }
    };

    Ok(format!("var({css_var})"))
}

pub(super) fn is_alias(value: &str) -> bool {
    alias_reference(value).is_some()
}

fn alias_reference(value: &str) -> Option<&str> {
    value
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .filter(|value| !value.is_empty())
}

fn alias_suffix(
    reference: &str,
    token_path: &str,
    namespace: &str,
    segments: &[&str],
    mode: NameMode,
) -> Result<String, String> {
    if segments.is_empty() {
        return Err(format!(
            "invalid alias for `{token_path}` in `{reference}`: `{namespace}` alias is missing a token name"
        ));
    }

    let mut names = Vec::new();
    for segment in segments {
        names.push(css_name_suffix_for_token(
            reference, token_path, segment, mode,
        )?);
    }

    Ok(names.join("-"))
}
