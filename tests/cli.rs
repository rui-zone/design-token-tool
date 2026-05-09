use std::fs;
use std::path::Path;
use std::process::Command;

const SNAPSHOT_FILES: &[&str] = &[
    "foundation/spacing.tokens.json",
    "foundation/radius.tokens.json",
    "foundation/typography.tokens.json",
    "foundation/colors.tokens.json",
    "theme/light.tokens.json",
    "theme/dark.tokens.json",
    "tokens.resolver.json",
    "theme.css",
];

#[test]
fn converts_fixture_design_md_to_dtcg_resolver_files_and_tailwind_v4_css() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_dir =
        std::env::temp_dir().join(format!("design-token-tool-{}-tokens", std::process::id()));
    let _ = fs::remove_dir_all(&output_dir);

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .arg("md-to-dtcg")
        .arg("--input")
        .arg(fixture_path(manifest_dir))
        .arg("--output")
        .arg(&output_dir)
        .status()
        .expect("CLI should run");

    assert!(status.success());

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .arg("dtcg-to-tailwind-v4")
        .arg("--resolver")
        .arg(output_dir.join("tokens.resolver.json"))
        .arg("--output")
        .arg(&output_dir)
        .status()
        .expect("CLI should run");

    assert!(status.success());

    insta::assert_snapshot!("cli_resolver_files", render_output_files(&output_dir));

    let _ = fs::remove_dir_all(output_dir);
}

#[test]
fn converts_fixture_design_md_directly_to_tailwind_v4_css() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let two_step_output_dir = std::env::temp_dir().join(format!(
        "design-token-tool-{}-two-step-tokens",
        std::process::id()
    ));
    let direct_output_dir = std::env::temp_dir().join(format!(
        "design-token-tool-{}-direct-tailwind",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&two_step_output_dir);
    let _ = fs::remove_dir_all(&direct_output_dir);

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .arg("md-to-dtcg")
        .arg("--input")
        .arg(fixture_path(manifest_dir))
        .arg("--output")
        .arg(&two_step_output_dir)
        .status()
        .expect("CLI should run");

    assert!(status.success());

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .arg("dtcg-to-tailwind-v4")
        .arg("--resolver")
        .arg(two_step_output_dir.join("tokens.resolver.json"))
        .arg("--output")
        .arg(&two_step_output_dir)
        .status()
        .expect("CLI should run");

    assert!(status.success());

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .arg("md-to-tailwind-v4")
        .arg("--input")
        .arg(fixture_path(manifest_dir))
        .arg("--output")
        .arg(&direct_output_dir)
        .status()
        .expect("CLI should run");

    assert!(status.success());

    let two_step_css = fs::read_to_string(two_step_output_dir.join("theme.css"))
        .expect("two-step Tailwind CSS should exist");
    let direct_css = fs::read_to_string(direct_output_dir.join("theme.css"))
        .expect("direct Tailwind CSS should exist");

    assert_eq!(direct_css, two_step_css);
    assert!(!direct_output_dir.join("tokens.resolver.json").exists());
    assert!(!direct_output_dir.join("foundation").exists());
    assert!(!direct_output_dir.join("theme").exists());

    let _ = fs::remove_dir_all(two_step_output_dir);
    let _ = fs::remove_dir_all(direct_output_dir);
}

#[test]
fn uses_default_paths_for_cli_options() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let default_output_dir = std::env::temp_dir().join(format!(
        "design-token-tool-{}-default-options",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&default_output_dir);
    fs::create_dir_all(&default_output_dir).expect("temp directory should be created");
    fs::copy(
        fixture_path(manifest_dir),
        default_output_dir.join("DESIGN.md"),
    )
    .expect("fixture should be copied to default input path");

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .current_dir(&default_output_dir)
        .arg("md-to-dtcg")
        .status()
        .expect("CLI should run");

    assert!(status.success());
    assert!(
        default_output_dir
            .join("tokens/tokens.resolver.json")
            .exists()
    );

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .current_dir(&default_output_dir)
        .arg("dtcg-to-tailwind-v4")
        .status()
        .expect("CLI should run");

    assert!(status.success());
    assert!(default_output_dir.join("tokens/theme.css").exists());

    let _ = fs::remove_dir_all(default_output_dir.join("styles"));

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .current_dir(&default_output_dir)
        .arg("md-to-tailwind-v4")
        .status()
        .expect("CLI should run");

    assert!(status.success());
    assert!(default_output_dir.join("styles/theme.css").exists());
    assert!(
        !default_output_dir
            .join("styles/tokens.resolver.json")
            .exists()
    );
    assert!(!default_output_dir.join("styles/foundation").exists());
    assert!(!default_output_dir.join("styles/theme").exists());

    let _ = fs::remove_dir_all(default_output_dir);
}

fn fixture_path(manifest_dir: &str) -> String {
    format!("{manifest_dir}/tests/fixtures/design.md")
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
