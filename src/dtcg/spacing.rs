use indexmap::IndexMap;
use serde_json::{Map, Value, json};

use crate::dtcg::alias::{is_alias, normalize_alias_for_foundation};
use crate::dtcg::dimension::{parse_dimension, yaml_number_to_json_value};

pub(super) fn build_spacing_tokens(
    spacing: IndexMap<String, serde_yaml::Value>,
) -> Result<Value, String> {
    let mut spacing_group = Map::new();
    let spacing_types = infer_spacing_types(&spacing)?;

    for (name, raw_value) in spacing {
        let token = match raw_value {
            serde_yaml::Value::Number(number) => json!({
                "$type": "number",
                "$value": yaml_number_to_json_value(&name, &number)?,
            }),
            serde_yaml::Value::String(value) if is_alias(&value) => {
                let token_type = spacing_types
                    .get(&name)
                    .ok_or_else(|| format!("failed to infer spacing alias type for `{name}`"))?;

                json!({
                    "$type": token_type,
                    "$value": normalize_alias_for_foundation(&name, &value)?,
                })
            }
            serde_yaml::Value::String(value) => json!({
                "$type": "dimension",
                "$value": parse_dimension(&name, &value)?,
            }),
            _ => {
                return Err(format!(
                    "invalid spacing value for `{name}`: expected number, px/rem dimension, or alias"
                ));
            }
        };

        spacing_group.insert(name, token);
    }

    let mut root = Map::new();
    root.insert("spacing".to_string(), Value::Object(spacing_group));
    Ok(Value::Object(root))
}

fn infer_spacing_types(
    spacing: &IndexMap<String, serde_yaml::Value>,
) -> Result<IndexMap<String, &'static str>, String> {
    let mut spacing_types = IndexMap::new();

    for (name, raw_value) in spacing {
        let token_type = match raw_value {
            serde_yaml::Value::Number(_) => "number",
            serde_yaml::Value::String(value) if is_alias(value) => {
                let reference_name = spacing_alias_target_name(value).ok_or_else(|| {
                    format!(
                        "invalid spacing alias for `{name}`: expected reference to `{{spacing.xxx}}`"
                    )
                })?;

                let Some(target_value) = spacing.get(reference_name) else {
                    return Err(format!(
                        "invalid spacing alias for `{name}`: unknown spacing token `{reference_name}`"
                    ));
                };

                match target_value {
                    serde_yaml::Value::Number(_) => "number",
                    serde_yaml::Value::String(target) if !is_alias(target) => "dimension",
                    serde_yaml::Value::String(_) => {
                        return Err(format!(
                            "invalid spacing alias for `{name}`: chained spacing aliases are not supported"
                        ));
                    }
                    _ => {
                        return Err(format!(
                            "invalid spacing alias for `{name}`: target `{reference_name}` has unsupported value"
                        ));
                    }
                }
            }
            serde_yaml::Value::String(_) => "dimension",
            _ => {
                return Err(format!(
                    "invalid spacing value for `{name}`: expected number, px/rem dimension, or alias"
                ));
            }
        };

        spacing_types.insert(name.clone(), token_type);
    }

    Ok(spacing_types)
}

fn spacing_alias_target_name(alias: &str) -> Option<&str> {
    alias
        .strip_prefix("{spacing.")
        .and_then(|value| value.strip_suffix('}'))
        .filter(|value| !value.is_empty() && !value.contains('.'))
}

pub(super) fn build_radius_tokens(
    rounded: IndexMap<String, serde_yaml::Value>,
) -> Result<Value, String> {
    let mut radius_group = Map::new();
    radius_group.insert("$type".to_string(), Value::String("dimension".to_string()));

    for (name, raw_value) in rounded {
        let value = match raw_value {
            serde_yaml::Value::String(value) if is_alias(&value) => {
                Value::String(normalize_alias_for_foundation(&name, &value)?)
            }
            serde_yaml::Value::String(value) => parse_dimension(&name, &value)?,
            _ => {
                return Err(format!(
                    "invalid rounded value for `{name}`: expected px/rem dimension or alias"
                ));
            }
        };

        radius_group.insert(name, json!({ "$value": value }));
    }

    let mut root = Map::new();
    root.insert("radius".to_string(), Value::Object(radius_group));
    Ok(Value::Object(root))
}
