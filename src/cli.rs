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
    MdToDtcg {
        #[arg(long, default_value = "DESIGN.md")]
        input: PathBuf,
        #[arg(long, default_value = "tokens")]
        output: PathBuf,
    },
    /// Generate Tailwind CSS v4 theme variables directly from DESIGN.md-compatible front matter.
    MdToTailwindV4 {
        #[arg(long, default_value = "DESIGN.md")]
        input: PathBuf,
        #[arg(long, default_value = "styles")]
        output: PathBuf,
    },
    /// Generate Tailwind CSS v4 theme variables from a DTCG resolver token directory.
    DtcgToTailwindV4 {
        #[arg(long, default_value = "tokens/tokens.resolver.json")]
        resolver: PathBuf,
        #[arg(long, default_value = "tokens")]
        output: PathBuf,
    },
}

#[derive(Debug)]
struct MdToDtcgArgs {
    input: PathBuf,
    output: PathBuf,
}

#[derive(Debug)]
struct MdToTailwindV4Args {
    input: PathBuf,
    output: PathBuf,
}

#[derive(Debug)]
struct DtcgToTailwindV4Args {
    resolver: PathBuf,
    output: PathBuf,
}

/// Parses command-line arguments and dispatches to the selected subcommand.
pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::MdToDtcg { input, output } => md_to_dtcg(MdToDtcgArgs { input, output }),
        Command::MdToTailwindV4 { input, output } => {
            md_to_tailwind_v4(MdToTailwindV4Args { input, output })
        }
        Command::DtcgToTailwindV4 { resolver, output } => {
            dtcg_to_tailwind_v4(DtcgToTailwindV4Args { resolver, output })
        }
    }
}

/// Converts a Markdown file with DESIGN.md-compatible front matter into DTCG resolver files.
fn md_to_dtcg(args: MdToDtcgArgs) -> Result<(), String> {
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

/// Converts a Markdown file with DESIGN.md-compatible front matter into Tailwind CSS v4 theme variables.
fn md_to_tailwind_v4(args: MdToTailwindV4Args) -> Result<(), String> {
    let source = fs::read_to_string(&args.input)
        .map_err(|error| format!("failed to read {}: {error}", args.input.display()))?;
    let output = convert_markdown_to_dtcg(&source)?;

    let resolver_file = output
        .files
        .iter()
        .find(|file| file.path == "tokens.resolver.json")
        .ok_or_else(|| "generated DTCG output is missing tokens.resolver.json".to_string())?;
    let resolver_source = serialize_json(resolver_file.path, &resolver_file.json)?;

    let css = convert_resolver_to_tailwind_v4(&resolver_source, |reference| {
        let token_file = output
            .files
            .iter()
            .find(|file| file.path == reference)
            .ok_or_else(|| format!("generated DTCG output is missing `{reference}`"))?;

        serialize_json(token_file.path, &token_file.json)
    })?;

    write_tailwind_css(&args.output, css)
}

/// Converts a DTCG resolver token directory into Tailwind CSS v4 theme variables.
fn dtcg_to_tailwind_v4(args: DtcgToTailwindV4Args) -> Result<(), String> {
    let resolver_source = fs::read_to_string(&args.resolver)
        .map_err(|error| format!("failed to read {}: {error}", args.resolver.display()))?;
    let resolver_dir = args.resolver.parent().unwrap_or_else(|| Path::new("."));

    let css = convert_resolver_to_tailwind_v4(&resolver_source, |reference| {
        let path = resolver_dir.join(reference);
        fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))
    })?;

    write_tailwind_css(&args.output, css)
}

fn serialize_json(path: &str, json: &serde_json::Value) -> Result<String, String> {
    serde_json::to_string(json).map_err(|error| format!("failed to serialize {path}: {error}"))
}

fn write_tailwind_css(output: &Path, css: String) -> Result<(), String> {
    fs::create_dir_all(output)
        .map_err(|error| format!("failed to create {}: {error}", output.display()))?;

    let output_path = output.join(TAILWIND_V4_THEME_FILE);
    fs::write(&output_path, css)
        .map_err(|error| format!("failed to write {}: {error}", output_path.display()))?;

    Ok(())
}
