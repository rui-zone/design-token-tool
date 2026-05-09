use serde_json::{Map, Value};

pub(super) fn validate_fields(
    reference: &str,
    token_path: &str,
    value: &Map<String, Value>,
) -> Result<(), String> {
    for property in value.keys() {
        if !is_supported_typography_field(property) {
            return Err(format!(
                "unsupported typography field `{property}` for token `{token_path}` in `{reference}`"
            ));
        }
    }

    Ok(())
}

fn is_supported_typography_field(property: &str) -> bool {
    matches!(
        property,
        "fontFamily"
            | "fontSize"
            | "fontWeight"
            | "letterSpacing"
            | "lineHeight"
            | "fontFeature"
            | "fontVariation"
    )
}
