use indexmap::IndexMap;
use serde_json::{Map, Value, json};

use crate::color::parse_color_token_value;
use crate::dtcg::Theme;
use crate::dtcg::alias::{normalize_alias_for_foundation, normalize_alias_for_theme};

pub(super) struct SplitColors {
    pub(super) foundation: IndexMap<String, Value>,
    pub(super) light: IndexMap<String, Value>,
    pub(super) dark: IndexMap<String, Value>,
}

pub(super) fn split_colors(colors: IndexMap<String, String>) -> Result<SplitColors, String> {
    let mut foundation_raw = IndexMap::new();
    let mut light_raw = IndexMap::new();
    let mut dark_raw = IndexMap::new();

    for (name, raw_value) in colors {
        if let Some(base_name) = name.strip_suffix(Theme::Light.suffix()) {
            light_raw.insert(base_name.to_string(), (name, raw_value));
        } else if let Some(base_name) = name.strip_suffix(Theme::Dark.suffix()) {
            dark_raw.insert(base_name.to_string(), (name, raw_value));
        } else {
            foundation_raw.insert(name, raw_value);
        }
    }

    for base_name in light_raw.keys() {
        if !dark_raw.contains_key(base_name) {
            return Err(format!(
                "missing dark theme color pair for `{}`",
                light_raw[base_name].0
            ));
        }
    }

    for base_name in dark_raw.keys() {
        if !light_raw.contains_key(base_name) {
            return Err(format!(
                "missing light theme color pair for `{}`",
                dark_raw[base_name].0
            ));
        }
    }

    for base_name in light_raw.keys() {
        if foundation_raw.contains_key(base_name) {
            return Err(format!(
                "foundation color `{base_name}` conflicts with themed color `{}`",
                light_raw[base_name].0
            ));
        }
    }

    let mut foundation = IndexMap::new();
    for (name, raw_value) in foundation_raw {
        let value = normalize_alias_for_foundation(&name, &raw_value)?;
        foundation.insert(name.clone(), parse_color_token_value(&name, &value)?);
    }

    let light = build_theme_colors(light_raw, Theme::Light)?;
    let dark = build_theme_colors(dark_raw, Theme::Dark)?;

    Ok(SplitColors {
        foundation,
        light,
        dark,
    })
}

pub(super) fn build_color_tokens(colors: IndexMap<String, Value>) -> Value {
    let mut color_group = Map::new();
    color_group.insert("$type".to_string(), Value::String("color".to_string()));

    for (name, value) in colors {
        color_group.insert(name, json!({ "$value": value }));
    }

    let mut root = Map::new();
    root.insert("colors".to_string(), Value::Object(color_group));
    Value::Object(root)
}

fn build_theme_colors(
    raw_colors: IndexMap<String, (String, String)>,
    theme: Theme,
) -> Result<IndexMap<String, Value>, String> {
    let mut colors = IndexMap::new();

    for (base_name, (source_name, raw_value)) in raw_colors {
        let value = normalize_alias_for_theme(&source_name, &raw_value, theme)?;
        colors.insert(base_name, parse_color_token_value(&source_name, &value)?);
    }

    Ok(colors)
}
