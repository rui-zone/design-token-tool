use std::fs;
use std::path::Path;
use std::process::Command;

const SNAPSHOT_FILES: &[&str] = &[
    "foundation/spacing.tokens.json",
    "foundation/radius.tokens.json",
    "foundation/typography.tokens.json",
    "foundation/semantic-colors.tokens.json",
    "theme/light.tokens.json",
    "theme/dark.tokens.json",
    "tokens.resolver.json",
];

#[test]
fn converts_fixture_design_md_to_dtcg_resolver_files() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_dir =
        std::env::temp_dir().join(format!("design-token-tool-{}-tokens", std::process::id()));
    let _ = fs::remove_dir_all(&output_dir);

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .arg("parse-md")
        .arg(format!("{manifest_dir}/tests/fixtures/design.md"))
        .arg(&output_dir)
        .status()
        .expect("CLI should run");

    assert!(status.success());

    insta::assert_snapshot!("cli_resolver_files", render_output_files(&output_dir));

    let _ = fs::remove_dir_all(output_dir);
}

fn render_output_files(output_dir: &Path) -> String {
    let mut snapshot = String::new();

    for path in SNAPSHOT_FILES {
        let file = fs::read_to_string(output_dir.join(path))
            .unwrap_or_else(|error| panic!("failed to read generated `{path}`: {error}"));

        snapshot.push_str("== ");
        snapshot.push_str(path);
        snapshot.push_str(" ==\n");
        snapshot.push_str(&file);
    }

    snapshot
}
