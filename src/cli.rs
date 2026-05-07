use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use design_token_tool::convert_markdown_to_dtcg;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Parse DESIGN.md-compatible front matter and write DTCG design token JSON.
    ParseMd { input: PathBuf, output: PathBuf },
}

#[derive(Debug)]
struct ParseMdArgs {
    input: PathBuf,
    output: PathBuf,
}

/// Parses command-line arguments and dispatches to the selected subcommand.
pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::ParseMd { input, output } => parse_md(ParseMdArgs { input, output }),
    }
}

/// Converts a Markdown file with DESIGN.md-compatible front matter into a DTCG JSON file.
fn parse_md(args: ParseMdArgs) -> Result<(), String> {
    let source = fs::read_to_string(&args.input)
        .map_err(|error| format!("failed to read {}: {error}", args.input.display()))?;
    let output = convert_markdown_to_dtcg(&source)?;
    let json = serde_json::to_string_pretty(&output)
        .map_err(|error| format!("failed to serialize DTCG JSON: {error}"))?;

    fs::write(&args.output, format!("{json}\n"))
        .map_err(|error| format!("failed to write {}: {error}", args.output.display()))?;

    Ok(())
}
