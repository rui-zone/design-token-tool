use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::{Map, Value, json};

use crate::color::parse_color_token_value;
use crate::front_matter::extract_front_matter;

#[derive(Debug, Deserialize)]
struct DesignFrontMatter {
    colors: Option<IndexMap<String, String>>,
}

/// Converts a Markdown document with DESIGN.md-compatible front matter into DTCG JSON.
pub fn convert_markdown_to_dtcg(markdown: &str) -> Result<Value, String> {
    let front_matter = extract_front_matter(markdown)?;
    let design: DesignFrontMatter = serde_yaml::from_str(front_matter)
        .map_err(|error| format!("failed to parse YAML front matter: {error}"))?;
    let colors = design
        .colors
        .ok_or_else(|| "missing required `colors` section".to_string())?;

    let mut color_group = Map::new();
    color_group.insert("$type".to_string(), Value::String("color".to_string()));

    for (name, raw_value) in colors {
        let value = parse_color_token_value(&name, &raw_value)?;
        color_group.insert(name, json!({ "$value": value }));
    }

    let mut root = Map::new();
    root.insert("colors".to_string(), Value::Object(color_group));
    Ok(Value::Object(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_colors_to_dtcg() {
        let markdown = "---\ncolors:\n  neutral-0: \"#ffffff\"\n  background-light: '{colors.neutral-20}'\n---\n";
        let output = convert_markdown_to_dtcg(markdown).expect("conversion should work");

        assert_eq!(output["colors"]["$type"], "color");
        assert_eq!(
            output["colors"]["neutral-0"]["$value"]["colorSpace"],
            "srgb"
        );
        assert_eq!(
            output["colors"]["neutral-0"]["$value"]["components"][0],
            1.0
        );
        assert_eq!(output["colors"]["neutral-0"]["$value"]["hex"], "#ffffff");
        assert_eq!(
            output["colors"]["background-light"]["$value"],
            "{colors.neutral-20}"
        );
    }
}
