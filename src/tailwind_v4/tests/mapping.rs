use super::convert;

#[test]
fn maps_typography_to_tailwind_text_variables() {
    let css = convert(&[
        (
            "foundation.tokens.json",
            r#"{
              "typography": {
                "$type": "typography",
                "body-md": {
                  "$value": {
                    "fontFamily": "Public Sans",
                    "fontSize": { "value": 16.0, "unit": "px" },
                    "fontWeight": 400,
                    "lineHeight": 1.6,
                    "letterSpacing": { "value": 0.0, "unit": "px" }
                  }
                }
              }
            }"#,
        ),
        ("light.tokens.json", "{}"),
        ("dark.tokens.json", "{}"),
    ])
    .expect("tailwind CSS should generate");

    assert!(css.contains("--font-body_md: Public Sans;"));
    assert!(css.contains("--text-body_md: 16px;"));
    assert!(css.contains("--text-body_md--line-height: 1.6;"));
    assert!(css.contains("--text-body_md--letter-spacing: 0px;"));
    assert!(css.contains("--text-body_md--font-weight: normal;"));
}

#[test]
fn maps_aliases_and_dark_overrides() {
    let css = convert(&[
        (
            "foundation.tokens.json",
            r##"{
              "colors": {
                "$type": "color",
                "neutral-0": {
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
        (
            "light.tokens.json",
            r##"{
              "colors": {
                "$type": "color",
                "background": { "$value": "{colors.neutral-0}" }
              }
            }"##,
        ),
        (
            "dark.tokens.json",
            r##"{
              "colors": {
                "$type": "color",
                "background": {
                  "$value": {
                    "colorSpace": "srgb",
                    "components": [0.0, 0.0, 0.0],
                    "alpha": 0.5,
                    "hex": "#000000"
                  }
                }
              }
            }"##,
        ),
    ])
    .expect("tailwind CSS should generate");

    assert!(css.contains("--color-background: var(--color-neutral-0);"));
    assert!(css.contains(".dark,\n[data-theme=\"dark\"] {"));
    assert!(css.contains("--color-background: rgb(0 0 0 / 0.5);"));
}

#[test]
fn maps_oklch_colors_to_css() {
    let css = convert(&[
        (
            "foundation.tokens.json",
            r##"{
              "colors": {
                "$type": "color",
                "brand": {
                  "$value": {
                    "colorSpace": "oklch",
                    "components": [0.5, 0.1, 250.0],
                    "alpha": 1.0
                  }
                }
              }
            }"##,
        ),
        (
            "light.tokens.json",
            r##"{
              "colors": {
                "$type": "color",
                "background": {
                  "$value": {
                    "colorSpace": "oklch",
                    "components": [0.6, 0.05, 180.0],
                    "alpha": 0.5
                  }
                }
              }
            }"##,
        ),
        ("dark.tokens.json", "{}"),
    ])
    .expect("tailwind CSS should generate");

    assert!(css.contains("--color-brand: oklch(50% 0.1 250deg);"));
    assert!(css.contains("--color-background: oklch(60% 0.05 180deg / 0.5);"));
}
