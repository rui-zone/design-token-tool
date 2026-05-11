use super::convert_resolver_to_figma;

const RESOLVER: &str = r##"{
  "sets": {
    "foundation": {
      "sources": [
        { "$ref": "foundation/spacing.tokens.json" },
        { "$ref": "foundation/typography.tokens.json" },
        { "$ref": "foundation/colors.tokens.json" }
      ]
    }
  },
  "modifiers": {
    "theme": {
      "contexts": {
        "light": [{ "$ref": "theme/light.tokens.json" }],
        "dark": [{ "$ref": "theme/dark.tokens.json" }]
      }
    },
    "page": {
      "contexts": {
        "compact": [{ "$ref": "page/compact.tokens.json" }]
      }
    }
  }
}"##;

#[test]
fn outputs_collections_and_modes() {
    let output = convert_resolver_to_figma(RESOLVER, load_ref).expect("conversion should work");

    let paths = output
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        paths,
        [
            "Foundation/default.tokens.json",
            "Theme/light.tokens.json",
            "Theme/dark.tokens.json",
            "Page/compact.tokens.json"
        ]
    );

    let foundation = &output.files[0].json;
    assert_eq!(
        foundation
            .pointer("/typography/body-md/fontFamily/$type")
            .and_then(|value| value.as_str()),
        Some("fontFamily")
    );
    assert_eq!(
        foundation
            .pointer("/typography/body-md/fontSize/$type")
            .and_then(|value| value.as_str()),
        Some("dimension")
    );

    let light = &output.files[1].json;
    assert_eq!(
        light
            .pointer("/colors/background/$value/hex")
            .and_then(|value| value.as_str()),
        Some("#fafafa")
    );
    assert_eq!(
        light
            .pointer("/colors/background/$extensions/com.figma.aliasData/targetVariableSetName")
            .and_then(|value| value.as_str()),
        Some("Foundation")
    );
    assert_eq!(
        light
            .pointer("/colors/background/$extensions/com.figma.aliasData/targetVariableName")
            .and_then(|value| value.as_str()),
        Some("colors/neutral-10")
    );
}

#[test]
fn rejects_rem_dimensions() {
    let resolver = r#"{
      "sets": { "foundation": { "sources": [{ "$ref": "foundation/spacing.tokens.json" }] } },
      "modifiers": {}
    }"#;
    let error = convert_resolver_to_figma(resolver, |_| {
        Ok(r#"{
          "spacing": {
            "$type": "dimension",
            "md": { "$value": { "value": 1, "unit": "rem" } }
          }
        }"#
        .to_string())
    })
    .expect_err("rem should fail");

    assert!(error.contains("spacing.md"));
    assert!(error.contains("foundation/spacing.tokens.json"));
    assert!(error.contains("px only"));
}

fn load_ref(reference: &str) -> Result<String, String> {
    match reference {
        "foundation/spacing.tokens.json" => Ok(r#"{
          "spacing": {
            "$type": "dimension",
            "sm": { "$value": { "value": 8, "unit": "px" } }
          }
        }"#
        .to_string()),
        "foundation/typography.tokens.json" => Ok(r#"{
          "typography": {
            "$type": "typography",
            "body-md": {
              "$value": {
                "fontFamily": "Public Sans",
                "fontSize": { "value": 16, "unit": "px" },
                "lineHeight": 1.6,
                "letterSpacing": { "value": 0, "unit": "px" },
                "fontWeight": 400
              }
            }
          }
        }"#
        .to_string()),
        "foundation/colors.tokens.json" => Ok(r##"{
          "colors": {
            "$type": "color",
            "neutral-10": {
              "$value": {
                "colorSpace": "srgb",
                "components": [0.98, 0.98, 0.98],
                "alpha": 1,
                "hex": "#fafafa"
              }
            }
          }
        }"##
        .to_string()),
        "theme/light.tokens.json" => Ok(r#"{
          "colors": {
            "$type": "color",
            "background": { "$value": "{colors.neutral-10}" }
          }
        }"#
        .to_string()),
        "theme/dark.tokens.json" => Ok(r##"{
          "colors": {
            "$type": "color",
            "background": {
              "$value": {
                "colorSpace": "srgb",
                "components": [0.07, 0.07, 0.07],
                "alpha": 1,
                "hex": "#111111"
              }
            }
          }
        }"##
        .to_string()),
        "page/compact.tokens.json" => Ok(r#"{
          "spacing": {
            "$type": "dimension",
            "page-padding": { "$value": { "value": 16, "unit": "px" } }
          }
        }"#
        .to_string()),
        _ => Err("missing ref".to_string()),
    }
}
