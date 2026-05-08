use indexmap::IndexMap;
use serde_json::{Map, Value, json};

use crate::dtcg::alias::{is_alias, normalize_alias_for_foundation};
use crate::dtcg::dimension::{parse_dimension, yaml_number_to_json_value};

pub(super) fn build_typography_tokens(
    typography: IndexMap<String, IndexMap<String, serde_yaml::Value>>,
) -> Result<Value, String> {
    let mut typography_group = Map::new();
    typography_group.insert("$type".to_string(), Value::String("typography".to_string()));

    for (name, raw_properties) in typography {
        let mut value = Map::new();

        for (property, raw_value) in raw_properties {
            let converted = match property.as_str() {
                "fontFamily" => parse_string_or_alias(&name, &property, raw_value)?,
                "fontSize" | "letterSpacing" => {
                    parse_dimension_property(&name, &property, raw_value)?
                }
                "fontWeight" => parse_font_weight(&name, raw_value)?,
                "lineHeight" => parse_line_height(&name, raw_value)?,
                "fontFeature" | "fontVariation" => {
                    parse_string_or_alias(&name, &property, raw_value)?
                }
                _ => {
                    return Err(format!(
                        "unsupported typography field `{property}` for `{name}`"
                    ));
                }
            };

            value.insert(property, converted);
        }

        validate_required_typography_fields(&name, &value)?;
        typography_group.insert(name, json!({ "$value": Value::Object(value) }));
    }

    let mut root = Map::new();
    root.insert("typography".to_string(), Value::Object(typography_group));
    Ok(Value::Object(root))
}

fn parse_string_or_alias(
    token_name: &str,
    property: &str,
    raw_value: serde_yaml::Value,
) -> Result<Value, String> {
    match raw_value {
        serde_yaml::Value::String(value) => Ok(Value::String(normalize_alias_for_foundation(
            token_name, &value,
        )?)),
        _ => Err(format!(
            "invalid typography field `{property}` for `{token_name}`: expected string"
        )),
    }
}

fn parse_dimension_property(
    token_name: &str,
    property: &str,
    raw_value: serde_yaml::Value,
) -> Result<Value, String> {
    match raw_value {
        serde_yaml::Value::String(value) if is_alias(&value) => Ok(Value::String(
            normalize_alias_for_foundation(token_name, &value)?,
        )),
        serde_yaml::Value::String(value) => {
            parse_dimension(&format!("typography.{token_name}.{property}"), &value)
        }
        _ => Err(format!(
            "invalid typography field `{property}` for `{token_name}`: expected px/rem dimension or alias"
        )),
    }
}

fn parse_font_weight(token_name: &str, raw_value: serde_yaml::Value) -> Result<Value, String> {
    match raw_value {
        serde_yaml::Value::Number(number) => {
            let value = yaml_number_to_json_value(token_name, &number)?;
            let Some(weight) = value.as_i64() else {
                return Err(format!(
                    "invalid fontWeight for `{token_name}`: expected integer 1..=1000"
                ));
            };

            if !(1..=1000).contains(&weight) {
                return Err(format!(
                    "invalid fontWeight for `{token_name}`: expected integer 1..=1000"
                ));
            }

            Ok(value)
        }
        serde_yaml::Value::String(value) if is_alias(&value) => Ok(Value::String(
            normalize_alias_for_foundation(token_name, &value)?,
        )),
        serde_yaml::Value::String(value) if is_valid_font_weight_alias(&value) => {
            Ok(Value::String(value))
        }
        serde_yaml::Value::String(value) => Err(format!(
            "invalid fontWeight for `{token_name}`: `{value}` is not a DTCG fontWeight value"
        )),
        _ => Err(format!(
            "invalid fontWeight for `{token_name}`: expected number, fontWeight string, or alias"
        )),
    }
}

fn parse_line_height(token_name: &str, raw_value: serde_yaml::Value) -> Result<Value, String> {
    match raw_value {
        serde_yaml::Value::Number(number) => yaml_number_to_json_value(token_name, &number),
        serde_yaml::Value::String(value) if is_alias(&value) => Ok(Value::String(
            normalize_alias_for_foundation(token_name, &value)?,
        )),
        _ => Err(format!(
            "invalid lineHeight for `{token_name}`: expected number or alias"
        )),
    }
}

fn validate_required_typography_fields(
    token_name: &str,
    value: &Map<String, Value>,
) -> Result<(), String> {
    for field in [
        "fontFamily",
        "fontSize",
        "fontWeight",
        "letterSpacing",
        "lineHeight",
    ] {
        if !value.contains_key(field) {
            return Err(format!(
                "missing required typography field `{field}` for `{token_name}`"
            ));
        }
    }

    Ok(())
}

fn is_valid_font_weight_alias(value: &str) -> bool {
    matches!(
        value,
        "thin"
            | "hairline"
            | "extra-light"
            | "ultra-light"
            | "light"
            | "normal"
            | "regular"
            | "book"
            | "medium"
            | "semi-bold"
            | "demi-bold"
            | "bold"
            | "extra-bold"
            | "ultra-bold"
            | "black"
            | "heavy"
            | "extra-black"
            | "ultra-black"
    )
}
