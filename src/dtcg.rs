use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;

use crate::front_matter::extract_front_matter;

mod alias;
mod colors;
mod dimension;
mod resolver;
mod spacing;
#[cfg(test)]
mod tests;
mod typography;

use colors::{build_color_tokens, split_colors};
use resolver::build_resolver;
use spacing::{build_radius_tokens, build_spacing_tokens};
use typography::build_typography_tokens;

#[derive(Debug, Deserialize)]
struct DesignFrontMatter {
    name: Option<String>,
    description: Option<String>,
    colors: Option<IndexMap<String, String>>,
    spacing: Option<IndexMap<String, serde_yaml::Value>>,
    rounded: Option<IndexMap<String, serde_yaml::Value>>,
    typography: Option<IndexMap<String, IndexMap<String, serde_yaml::Value>>>,
}

#[derive(Debug, PartialEq)]
pub struct GeneratedTokenFile {
    pub path: &'static str,
    pub json: Value,
}

#[derive(Debug, PartialEq)]
pub struct GeneratedTokens {
    pub files: Vec<GeneratedTokenFile>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn suffix(self) -> &'static str {
        match self {
            Self::Light => "-light",
            Self::Dark => "-dark",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// Converts a Markdown document with DESIGN.md-compatible front matter into DTCG token files.
pub fn convert_markdown_to_dtcg(markdown: &str) -> Result<GeneratedTokens, String> {
    let front_matter = extract_front_matter(markdown)?;
    let design: DesignFrontMatter = serde_yaml::from_str(front_matter)
        .map_err(|error| format!("failed to parse YAML front matter: {error}"))?;

    let colors = split_colors(design.colors.unwrap_or_default())?;

    Ok(GeneratedTokens {
        files: vec![
            GeneratedTokenFile {
                path: "foundation/spacing.tokens.json",
                json: build_spacing_tokens(design.spacing.unwrap_or_default())?,
            },
            GeneratedTokenFile {
                path: "foundation/radius.tokens.json",
                json: build_radius_tokens(design.rounded.unwrap_or_default())?,
            },
            GeneratedTokenFile {
                path: "foundation/typography.tokens.json",
                json: build_typography_tokens(design.typography.unwrap_or_default())?,
            },
            GeneratedTokenFile {
                path: "foundation/semantic-colors.tokens.json",
                json: build_color_tokens(colors.foundation),
            },
            GeneratedTokenFile {
                path: "theme/light.tokens.json",
                json: build_color_tokens(colors.light),
            },
            GeneratedTokenFile {
                path: "theme/dark.tokens.json",
                json: build_color_tokens(colors.dark),
            },
            GeneratedTokenFile {
                path: "tokens.resolver.json",
                json: build_resolver(design.name, design.description),
            },
        ],
    })
}
