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
    /// Parse DESIGN.md-compatible front matter and write DTCG resolver token files.
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

/// Converts a Markdown file with DESIGN.md-compatible front matter into DTCG resolver files.
fn parse_md(args: ParseMdArgs) -> Result<(), String> {
    let source = fs::read_to_string(&args.input)
        .map_err(|error| format!("failed to read {}: {error}", args.input.display()))?;
    let output = convert_markdown_to_dtcg(&source)?;

    for file in output.files {
        let output_path = args.output.join(file.path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }

        let json = serde_json::to_string_pretty(&file.json)
            .map_err(|error| format!("failed to serialize {}: {error}", output_path.display()))?;

        fs::write(&output_path, format!("{json}\n"))
            .map_err(|error| format!("failed to write {}: {error}", output_path.display()))?;
    }

    Ok(())
}
