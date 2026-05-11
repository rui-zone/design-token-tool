use std::collections::HashMap;

use serde_json::{Map, Value};

use super::GeneratedFigmaFile;
use super::token::{alias_reference, is_metadata_key};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct TokenTarget {
    collection: String,
    name: String,
    token_type: String,
    value: Value,
}

pub(super) fn build_alias_index(
    files: &[GeneratedFigmaFile],
) -> Result<HashMap<String, Vec<TokenTarget>>, String> {
    let mut index: HashMap<String, Vec<TokenTarget>> = HashMap::new();

    for file in files {
        let collection = file.path.split('/').next().unwrap_or("");
        collect_targets(&file.json, collection, &mut Vec::new(), None, &mut index)?;
    }

    Ok(index)
}

pub(super) fn add_alias_extensions(
    file_path: &str,
    value: &mut Value,
    alias_index: &HashMap<String, Vec<TokenTarget>>,
) -> Result<(), String> {
    let current_collection = file_path.split('/').next().unwrap_or("");
    add_alias_extensions_in_node(
        file_path,
        current_collection,
        value,
        &mut Vec::new(),
        None,
        alias_index,
    )
}

fn collect_targets(
    value: &Value,
    collection: &str,
    path: &mut Vec<String>,
    inherited_type: Option<&str>,
    index: &mut HashMap<String, Vec<TokenTarget>>,
) -> Result<(), String> {
    let Some(object) = value.as_object() else {
        return Ok(());
    };
    let node_type = object
        .get("$type")
        .and_then(Value::as_str)
        .or(inherited_type);

    if object.contains_key("$value") {
        let token_type = node_type
            .ok_or_else(|| format!("missing `$type` for Figma token `{}`", path.join(".")))?;
        let target = TokenTarget {
            collection: collection.to_string(),
            name: path.join("/"),
            token_type: token_type.to_string(),
            value: object
                .get("$value")
                .ok_or_else(|| format!("missing `$value` for Figma token `{}`", path.join(".")))?
                .clone(),
        };
        let entry = index.entry(path.join(".")).or_default();
        if !entry.contains(&target) {
            entry.push(target);
        }
        return Ok(());
    }

    for (key, child) in object {
        if is_metadata_key(key) {
            continue;
        }

        path.push(key.clone());
        collect_targets(child, collection, path, node_type, index)?;
        path.pop();
    }

    Ok(())
}

fn add_alias_extensions_in_node(
    file_path: &str,
    current_collection: &str,
    value: &mut Value,
    path: &mut Vec<String>,
    inherited_type: Option<&str>,
    alias_index: &HashMap<String, Vec<TokenTarget>>,
) -> Result<(), String> {
    let Some(object) = value.as_object_mut() else {
        return Ok(());
    };
    let node_type = object
        .get("$type")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .or_else(|| inherited_type.map(ToOwned::to_owned));

    if object.contains_key("$value") {
        let Some(alias) = object
            .get("$value")
            .and_then(Value::as_str)
            .and_then(alias_reference)
        else {
            return Ok(());
        };

        let current_type = node_type.as_deref().ok_or_else(|| {
            format!(
                "missing `$type` for alias token `{}` in `{file_path}`",
                path.join(".")
            )
        })?;
        let target = resolve_alias_target(alias, current_collection, current_type, alias_index)
            .ok_or_else(|| {
                format!(
                    "unresolved alias `{{{alias}}}` for `{}` in `{file_path}`",
                    path.join(".")
                )
            })?;

        if target.collection != current_collection {
            object.insert("$value".to_string(), target.value.clone());
            insert_alias_data(object, target);
        }

        return Ok(());
    }

    for (key, child) in object {
        if is_metadata_key(key) {
            continue;
        }

        path.push(key.clone());
        add_alias_extensions_in_node(
            file_path,
            current_collection,
            child,
            path,
            node_type.as_deref(),
            alias_index,
        )?;
        path.pop();
    }

    Ok(())
}

fn resolve_alias_target<'a>(
    alias: &str,
    current_collection: &str,
    current_type: &str,
    alias_index: &'a HashMap<String, Vec<TokenTarget>>,
) -> Option<&'a TokenTarget> {
    let targets = alias_index.get(alias)?;
    targets
        .iter()
        .find(|target| target.collection == current_collection && target.token_type == current_type)
        .or_else(|| {
            targets.iter().find(|target| {
                target.collection == "Foundation" && target.token_type == current_type
            })
        })
        .or_else(|| {
            let mut matching = targets
                .iter()
                .filter(|target| target.token_type == current_type);
            let first = matching.next()?;
            if matching.next().is_none() {
                Some(first)
            } else {
                None
            }
        })
}

fn insert_alias_data(token: &mut Map<String, Value>, target: &TokenTarget) {
    let extensions = token
        .entry("$extensions".to_string())
        .or_insert_with(|| Value::Object(Map::new()));

    let Value::Object(extensions) = extensions else {
        return;
    };

    let figma = extensions
        .entry("com.figma.aliasData".to_string())
        .or_insert_with(|| Value::Object(Map::new()));

    let Value::Object(figma) = figma else {
        return;
    };

    figma.insert(
        "targetVariableSetName".to_string(),
        Value::String(target.collection.clone()),
    );
    figma.insert(
        "targetVariableName".to_string(),
        Value::String(target.name.clone()),
    );
}
