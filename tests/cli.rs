use std::fs;
use std::process::Command;

use serde_json::Value;

#[test]
fn converts_fixture_design_md_to_dtcg_json() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_path = std::env::temp_dir().join(format!(
        "design-token-tool-{}-tokens.tokens.json",
        std::process::id()
    ));

    let status = Command::new(env!("CARGO_BIN_EXE_design-token-tool"))
        .arg("parse-md")
        .arg(format!("{manifest_dir}/tests/fixtures/design.md"))
        .arg(&output_path)
        .status()
        .expect("CLI should run");

    assert!(status.success());

    let output = fs::read_to_string(&output_path).expect("output JSON should be readable");
    let json: Value = serde_json::from_str(&output).expect("output should be valid JSON");

    assert_eq!(json["colors"]["$type"], "color");
    assert_eq!(json["colors"]["neutral-0"]["$value"]["colorSpace"], "srgb");
    assert_eq!(json["colors"]["neutral-0"]["$value"]["components"][0], 1.0);
    assert_eq!(json["colors"]["neutral-0"]["$value"]["components"][1], 1.0);
    assert_eq!(json["colors"]["neutral-0"]["$value"]["components"][2], 1.0);
    assert_eq!(json["colors"]["neutral-0"]["$value"]["alpha"], 1.0);
    assert_eq!(json["colors"]["neutral-0"]["$value"]["hex"], "#ffffff");
    assert_eq!(json["colors"]["black-alpha-20"]["$value"]["alpha"], 0.04);
    assert_eq!(
        json["colors"]["background-light"]["$value"],
        "{colors.neutral-10}"
    );

    let _ = fs::remove_file(output_path);
}
