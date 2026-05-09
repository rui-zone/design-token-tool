//! Library support for converting DESIGN.md-compatible front matter into DTCG resolver token files.

mod color;
mod dtcg;
mod front_matter;
mod tailwind_v4;

pub use dtcg::{GeneratedTokenFile, GeneratedTokens, convert_markdown_to_dtcg};
pub use tailwind_v4::{TAILWIND_V4_THEME_FILE, convert_resolver_to_tailwind_v4};
