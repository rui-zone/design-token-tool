//! Library support for converting DESIGN.md-compatible front matter into DTCG resolver token files.

mod color;
mod dtcg;
mod front_matter;

pub use dtcg::{GeneratedTokenFile, GeneratedTokens, convert_markdown_to_dtcg};
