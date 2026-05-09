use super::convert_resolver_to_tailwind_v4;

mod errors;
mod mapping;

const RESOLVER: &str = r#"{
  "sets": { "foundation": { "sources": [{ "$ref": "foundation.tokens.json" }] } },
  "modifiers": {
    "theme": {
      "contexts": {
        "light": [{ "$ref": "light.tokens.json" }],
        "dark": [{ "$ref": "dark.tokens.json" }]
      }
    }
  }
}"#;

pub(super) fn convert(files: &[(&str, &str)]) -> Result<String, String> {
    convert_resolver_to_tailwind_v4(RESOLVER, |reference| {
        files
            .iter()
            .find(|(path, _)| path == &reference)
            .map(|(_, source)| source.to_string())
            .ok_or_else(|| format!("missing test file {reference}"))
    })
}
