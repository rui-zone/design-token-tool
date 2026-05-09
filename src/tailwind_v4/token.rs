use serde_json::Value;

pub(super) struct TokenEntry<'a> {
    pub(super) name: String,
    pub(super) path: String,
    pub(super) value: &'a Value,
    pub(super) group_type: Option<&'a str>,
}

pub(super) fn token_entries<'a>(
    reference: &str,
    group: &str,
    value: &'a Value,
) -> Result<Vec<TokenEntry<'a>>, String> {
    let group_object = value.as_object().ok_or_else(|| {
        format!("invalid token group `{group}` in `{reference}`: expected object")
    })?;
    let group_type = group_object.get("$type").and_then(Value::as_str);
    let mut entries = Vec::new();

    for (name, value) in group_object {
        if is_metadata_key(name) {
            continue;
        }

        entries.push(TokenEntry {
            name: name.clone(),
            path: format!("{group}.{name}"),
            value,
            group_type,
        });
    }

    Ok(entries)
}

pub(super) fn token_type<'a>(token: &'a Value, group_type: Option<&'a str>) -> Option<&'a str> {
    token.get("$type").and_then(Value::as_str).or(group_type)
}

pub(super) fn ensure_token_type(
    reference: &str,
    token_path: &str,
    actual: Option<&str>,
    expected: &[&str],
) -> Result<(), String> {
    if actual.is_some_and(|token_type| expected.contains(&token_type)) {
        return Ok(());
    }

    Err(format!(
        "unsupported token type for `{token_path}` in `{reference}`: expected {}, got {}",
        expected.join(" or "),
        actual.unwrap_or("<missing>")
    ))
}

pub(super) fn token_value<'a>(
    reference: &str,
    token_path: &str,
    token: &'a Value,
) -> Result<&'a Value, String> {
    token
        .get("$value")
        .ok_or_else(|| format!("missing `$value` for `{token_path}` in `{reference}`"))
}

pub(super) fn is_metadata_key(key: &str) -> bool {
    key.starts_with('$')
}
