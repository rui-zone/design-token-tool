use std::fs;
use std::path::Path;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use design_token_tool::{
    TAILWIND_V4_THEME_FILE, convert_markdown_to_dtcg, convert_resolver_to_tailwind_v4,
};

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
    /// Generate Tailwind CSS v4 theme variables from a DTCG resolver token directory.
    GenTailwindV4 { resolver: PathBuf, output: PathBuf },
}

#[derive(Debug)]
struct ParseMdArgs {
    input: PathBuf,
    output: PathBuf,
}

#[derive(Debug)]
struct GenTailwindV4Args {
    resolver: PathBuf,
    output: PathBuf,
}

/// Parses command-line arguments and dispatches to the selected subcommand.
pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::ParseMd { input, output } => parse_md(ParseMdArgs { input, output }),
        Command::GenTailwindV4 { resolver, output } => {
            gen_tailwind_v4(GenTailwindV4Args { resolver, output })
        }
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

/// Converts a DTCG resolver token directory into Tailwind CSS v4 theme variables.
fn gen_tailwind_v4(args: GenTailwindV4Args) -> Result<(), String> {
    let resolver_source = fs::read_to_string(&args.resolver)
        .map_err(|error| format!("failed to read {}: {error}", args.resolver.display()))?;
    let resolver_dir = args.resolver.parent().unwrap_or_else(|| Path::new("."));

    let css = convert_resolver_to_tailwind_v4(&resolver_source, |reference| {
        let path = resolver_dir.join(reference);
        fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))
    })?;

    fs::create_dir_all(&args.output)
        .map_err(|error| format!("failed to create {}: {error}", args.output.display()))?;

    let output_path = args.output.join(TAILWIND_V4_THEME_FILE);
    fs::write(&output_path, css)
        .map_err(|error| format!("failed to write {}: {error}", output_path.display()))?;

    Ok(())
}
