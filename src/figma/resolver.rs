use serde_json::Value;

#[derive(Debug)]
pub(super) struct ResolverCollection {
    pub(super) name: String,
    pub(super) modes: Vec<ResolverMode>,
}

#[derive(Debug)]
pub(super) struct ResolverMode {
    pub(super) name: String,
    pub(super) refs: Vec<String>,
}

pub(super) fn collect_resolver_collections(
    resolver: &Value,
) -> Result<Vec<ResolverCollection>, String> {
    let mut collections = Vec::new();
    collections.push(ResolverCollection {
        name: "Foundation".to_string(),
        modes: vec![ResolverMode {
            name: "default".to_string(),
            refs: collect_ref_array(resolver, &["sets", "foundation", "sources"])?,
        }],
    });

    let modifiers = resolver
        .get("modifiers")
        .and_then(Value::as_object)
        .ok_or_else(|| "missing resolver `modifiers`".to_string())?;

    for (modifier_name, modifier) in modifiers {
        let contexts = modifier
            .get("contexts")
            .and_then(Value::as_object)
            .ok_or_else(|| format!("missing resolver `modifiers.{modifier_name}.contexts`"))?;

        let mut modes = Vec::new();
        for (context_name, value) in contexts {
            modes.push(ResolverMode {
                name: context_name.clone(),
                refs: collect_ref_array_from_value(
                    value,
                    &format!("modifiers.{modifier_name}.contexts.{context_name}"),
                )?,
            });
        }

        collections.push(ResolverCollection {
            name: collection_name(modifier_name)?,
            modes,
        });
    }

    Ok(collections)
}

fn collect_ref_array(resolver: &Value, path: &[&str]) -> Result<Vec<String>, String> {
    let path_label = path.join(".");
    let value = resolver
        .pointer(&resolver_pointer(path))
        .ok_or_else(|| format!("missing resolver `{path_label}`"))?;

    collect_ref_array_from_value(value, &path_label)
}

fn collect_ref_array_from_value(value: &Value, path_label: &str) -> Result<Vec<String>, String> {
    let values = value.as_array().ok_or_else(|| {
        format!("invalid resolver `{path_label}`: expected an array of $ref objects")
    })?;

    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            value
                .get("$ref")
                .and_then(Value::as_str)
                .filter(|reference| !reference.is_empty())
                .map(ToOwned::to_owned)
                .ok_or_else(|| format!("missing resolver $ref at {path_label}[{index}]"))
        })
        .collect()
}

fn resolver_pointer(path: &[&str]) -> String {
    let mut pointer = String::new();

    for segment in path {
        pointer.push('/');
        pointer.push_str(segment);
    }

    pointer
}

fn collection_name(name: &str) -> Result<String, String> {
    let mut collection = String::new();

    for part in name.split(|character: char| !character.is_ascii_alphanumeric()) {
        if part.is_empty() {
            continue;
        }

        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            collection.push(first.to_ascii_uppercase());
            for character in chars {
                collection.push(character);
            }
        }
    }

    if collection.is_empty() {
        return Err(format!(
            "invalid modifier name `{name}`: expected ASCII letters or numbers"
        ));
    }

    Ok(collection)
}
