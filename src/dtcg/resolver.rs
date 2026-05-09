use serde_json::{Map, Value, json};

const RESOLVER_SCHEMA: &str = "https://www.designtokens.org/schemas/2025.10/resolver.json";

pub(super) fn build_resolver(name: Option<String>, description: Option<String>) -> Value {
    let mut root = Map::new();
    root.insert(
        "$schema".to_string(),
        Value::String(RESOLVER_SCHEMA.to_string()),
    );

    if let Some(name) = name {
        root.insert("name".to_string(), Value::String(name));
    }

    root.insert("version".to_string(), Value::String("2025.10".to_string()));

    if let Some(description) = description {
        root.insert("description".to_string(), Value::String(description));
    }

    root.insert(
        "sets".to_string(),
        json!({
            "foundation": {
                "sources": [
                    { "$ref": "foundation/spacing.tokens.json" },
                    { "$ref": "foundation/radius.tokens.json" },
                    { "$ref": "foundation/typography.tokens.json" },
                    { "$ref": "foundation/colors.tokens.json" }
                ]
            }
        }),
    );
    root.insert(
        "modifiers".to_string(),
        json!({
            "theme": {
                "contexts": {
                    "light": [
                        { "$ref": "theme/light.tokens.json" }
                    ],
                    "dark": [
                        { "$ref": "theme/dark.tokens.json" }
                    ]
                },
                "default": "light"
            }
        }),
    );
    root.insert(
        "resolutionOrder".to_string(),
        json!([
            { "$ref": "#/sets/foundation" },
            { "$ref": "#/modifiers/theme" }
        ]),
    );

    Value::Object(root)
}
