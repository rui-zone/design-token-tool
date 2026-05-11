mod alias;
mod resolver;
#[cfg(test)]
mod tests;
mod token;
mod token_file;
mod typography;
mod values;

use serde_json::{Map, Value};

use alias::{add_alias_extensions, build_alias_index};
use resolver::collect_resolver_collections;
use token_file::merge_token_file;

#[derive(Debug, PartialEq)]
pub struct GeneratedFigmaFile {
    pub path: String,
    pub json: Value,
}

#[derive(Debug, PartialEq)]
pub struct GeneratedFigmaTokens {
    pub files: Vec<GeneratedFigmaFile>,
}

/// Converts a DTCG resolver and its referenced token files into Figma-importable mode files.
pub fn convert_resolver_to_figma<F>(
    resolver_source: &str,
    mut load_ref: F,
) -> Result<GeneratedFigmaTokens, String>
where
    F: FnMut(&str) -> Result<String, String>,
{
    let resolver: Value = serde_json::from_str(resolver_source)
        .map_err(|error| format!("failed to parse tokens resolver JSON: {error}"))?;
    let collections = collect_resolver_collections(&resolver)?;

    let mut files = Vec::new();
    for collection in collections {
        for mode in collection.modes {
            let mut root = Map::new();

            for reference in mode.refs {
                let source = load_ref(&reference).map_err(|error| {
                    format!("failed to load resolver $ref `{reference}`: {error}")
                })?;
                let token_file: Value = serde_json::from_str(&source).map_err(|error| {
                    format!("failed to parse token source `{reference}`: {error}")
                })?;
                merge_token_file(&reference, &token_file, &mut root)?;
            }

            files.push(GeneratedFigmaFile {
                path: format!("{}/{}.tokens.json", collection.name, mode.name),
                json: Value::Object(root),
            });
        }
    }

    let alias_index = build_alias_index(&files)?;
    for file in &mut files {
        add_alias_extensions(&file.path, &mut file.json, &alias_index)?;
    }

    Ok(GeneratedFigmaTokens { files })
}
