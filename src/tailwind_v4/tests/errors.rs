use super::convert;
use crate::convert_resolver_to_tailwind_v4;

#[test]
fn rejects_missing_resolver_ref() {
    let resolver = r#"{
      "sets": { "foundation": { "sources": [{}] } },
      "modifiers": { "theme": { "contexts": { "light": [], "dark": [] } } }
    }"#;

    let error = convert_resolver_to_tailwind_v4(resolver, |_| Ok("{}".to_string()))
        .expect_err("missing ref should fail");

    assert!(error.contains("$ref"));
    assert!(error.contains("sets.foundation.sources[0]"));
}

#[test]
fn rejects_unsupported_token_group() {
    let error = convert(&[
        ("foundation.tokens.json", r#"{ "shadow": {} }"#),
        ("light.tokens.json", "{}"),
        ("dark.tokens.json", "{}"),
    ])
    .expect_err("unsupported group should fail");

    assert!(error.contains("unsupported token group `shadow`"));
    assert!(error.contains("foundation.tokens.json"));
}

#[test]
fn rejects_invalid_token_name() {
    let error = convert(&[
        (
            "foundation.tokens.json",
            r##"{
              "colors": {
                "$type": "color",
                "brand/primary": {
                  "$value": {
                    "colorSpace": "srgb",
                    "components": [1.0, 1.0, 1.0],
                    "alpha": 1.0,
                    "hex": "#ffffff"
                  }
                }
              }
            }"##,
        ),
        ("light.tokens.json", "{}"),
        ("dark.tokens.json", "{}"),
    ])
    .expect_err("invalid name should fail");

    assert!(error.contains("invalid token name `brand/primary`"));
}
