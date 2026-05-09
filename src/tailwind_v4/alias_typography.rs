use super::names::{NameMode, css_name_suffix_for_token};

pub(super) fn typography_alias_css_var(
    reference: &str,
    token_path: &str,
    namespace: &str,
    segments: &[&str],
) -> Result<String, String> {
    if segments.len() != 2 {
        return Err(format!(
            "unsupported typography alias for `{token_path}` in `{reference}`: expected {{{namespace}.name.field}}"
        ));
    }

    let name = css_name_suffix_for_token(reference, token_path, segments[0], NameMode::Typography)?;
    match segments[1] {
        "fontFamily" => Ok(format!("--font-{name}")),
        "fontSize" => Ok(format!("--text-{name}")),
        "lineHeight" => Ok(format!("--text-{name}--line-height")),
        "letterSpacing" => Ok(format!("--text-{name}--letter-spacing")),
        "fontWeight" => Ok(format!("--text-{name}--font-weight")),
        field => Err(format!(
            "unsupported typography alias field `{field}` for `{token_path}` in `{reference}`"
        )),
    }
}
