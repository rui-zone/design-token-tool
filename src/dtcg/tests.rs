use super::*;

#[test]
fn rejects_missing_theme_color_pair() {
    let error = convert_markdown_to_dtcg("---\ncolors:\n  background-light: \"#ffffff\"\n---\n")
        .expect_err("missing dark pair should fail");

    assert!(error.contains("background-light"));
}

#[test]
fn rejects_foundation_color_conflict_with_themed_color() {
    let error = convert_markdown_to_dtcg(
        "---\ncolors:\n  background: \"#ffffff\"\n  background-light: \"#ffffff\"\n  background-dark: \"#000000\"\n---\n",
    )
    .expect_err("conflicting foundation and theme colors should fail");

    assert!(error.contains("background"));
}

#[test]
fn rejects_em_dimension() {
    let error =
        convert_markdown_to_dtcg("---\nspacing:\n  sm: 1em\n---\n").expect_err("em should fail");

    assert!(error.contains("px and rem"));
}

#[test]
fn rejects_typography_line_height_dimension() {
    let error = convert_markdown_to_dtcg(
        "---\ntypography:\n  body:\n    fontFamily: Public Sans\n    fontSize: 16px\n    fontWeight: 400\n    lineHeight: 24px\n    letterSpacing: 0px\n---\n",
    )
    .expect_err("dimension lineHeight should fail");

    assert!(error.contains("lineHeight"));
    assert!(error.contains("number or alias"));
}

#[test]
fn rejects_typography_missing_required_field() {
    let error = convert_markdown_to_dtcg(
        "---\ntypography:\n  body:\n    fontFamily: Public Sans\n    fontSize: 16px\n    fontWeight: 400\n    lineHeight: 1.5\n---\n",
    )
    .expect_err("missing typography field should fail");

    assert!(error.contains("letterSpacing"));
    assert!(error.contains("body"));
}

#[test]
fn infers_spacing_alias_type_from_target() {
    let output = convert_markdown_to_dtcg(
        "---\nspacing:\n  columns: 12\n  columns-copy: \"{spacing.columns}\"\n  sm: 8px\n  sm-copy: \"{spacing.sm}\"\n---\n",
    )
    .expect("spacing aliases should work");

    let spacing = &output.files[0].json["spacing"];
    assert_eq!(spacing["columns-copy"]["$type"], "number");
    assert_eq!(spacing["sm-copy"]["$type"], "dimension");
}

#[test]
fn rejects_spacing_alias_when_type_cannot_be_inferred() {
    let error = convert_markdown_to_dtcg("---\nspacing:\n  sm: \"{sizes.sm}\"\n---\n")
        .expect_err("untyped spacing alias should fail");

    assert!(error.contains("spacing alias"));
    assert!(error.contains("sm"));
}

#[test]
fn normalizes_same_theme_alias_and_rejects_cross_theme_alias() {
    let output = convert_markdown_to_dtcg(
        "---\ncolors:\n  text-light: \"#000000\"\n  text-dark: \"#ffffff\"\n  background-light: \"{colors.text-light}\"\n  background-dark: \"{colors.text-dark}\"\n---\n",
    )
    .expect("same-theme aliases should work");

    assert_eq!(
        output.files[4].json["colors"]["background"]["$value"],
        "{colors.text}"
    );
    assert_eq!(
        output.files[5].json["colors"]["background"]["$value"],
        "{colors.text}"
    );

    let error = convert_markdown_to_dtcg(
        "---\ncolors:\n  text-light: \"#000000\"\n  text-dark: \"#ffffff\"\n  background-light: \"{colors.text-dark}\"\n  background-dark: \"{colors.text-dark}\"\n---\n",
    )
    .expect_err("cross-theme aliases should fail");

    assert!(error.contains("background-light"));
}
