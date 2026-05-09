use serde_json::Value;

pub(super) struct ResolverRefs {
    pub(super) foundation: Vec<String>,
    pub(super) light: Vec<String>,
    pub(super) dark: Vec<String>,
}

pub(super) fn collect_resolver_refs(resolver: &Value) -> Result<ResolverRefs, String> {
    Ok(ResolverRefs {
        foundation: collect_ref_array(resolver, &["sets", "foundation", "sources"])?,
        light: collect_ref_array(resolver, &["modifiers", "theme", "contexts", "light"])?,
        dark: collect_ref_array(resolver, &["modifiers", "theme", "contexts", "dark"])?,
    })
}

fn collect_ref_array(resolver: &Value, path: &[&str]) -> Result<Vec<String>, String> {
    let path_label = path.join(".");
    let values = resolver
        .pointer(&resolver_pointer(path))
        .ok_or_else(|| format!("missing resolver `{path_label}`"))?;
    let values = values.as_array().ok_or_else(|| {
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
