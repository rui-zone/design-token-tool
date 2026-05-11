use serde_json::{Map, Value};

pub(super) fn token(token_type: &str, value: Value) -> Value {
    let mut token = Map::new();
    token.insert("$type".to_string(), Value::String(token_type.to_string()));
    token.insert("$value".to_string(), value);
    Value::Object(token)
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

pub(super) fn is_alias(value: &str) -> bool {
    alias_reference(value).is_some()
}

pub(super) fn alias_reference(value: &str) -> Option<&str> {
    value
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .filter(|value| !value.is_empty())
}

pub(super) fn is_metadata_key(key: &str) -> bool {
    key.starts_with('$')
}
