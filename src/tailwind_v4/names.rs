#[derive(Clone, Copy)]
pub(super) enum NameMode {
    Preserve,
    Typography,
}

pub(super) fn css_name_suffix_for_token(
    reference: &str,
    token_path: &str,
    name: &str,
    mode: NameMode,
) -> Result<String, String> {
    css_name_suffix(name, mode)
        .map_err(|error| format!("{error} for `{token_path}` in `{reference}`"))
}

fn css_name_suffix(name: &str, mode: NameMode) -> Result<String, String> {
    if name.is_empty() {
        return Err("invalid token name: expected a non-empty name".to_string());
    }

    let mut suffix = String::new();
    for character in name.chars() {
        let mapped = match mode {
            NameMode::Preserve => character,
            NameMode::Typography if character == '-' => '_',
            NameMode::Typography => character,
        };

        if !mapped.is_ascii_alphanumeric() && mapped != '-' && mapped != '_' {
            return Err(format!(
                "invalid token name `{name}`: expected ASCII letters, numbers, `_`, or `-`"
            ));
        }

        suffix.push(mapped);
    }

    Ok(suffix)
}
