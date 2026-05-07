//! Library support for converting DESIGN.md-compatible front matter into DTCG design token JSON.

mod color;
mod dtcg;
mod front_matter;

pub use dtcg::convert_markdown_to_dtcg;
