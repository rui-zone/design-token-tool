## Project Scope

This is a Rust CLI that parses DESIGN.md-compatible YAML front matter from Markdown files and generates DTCG design token JSON plus a DTCG resolver file.

References:

- DESIGN.md: https://github.com/google-labs-code/design.md/blob/main/docs/spec.md
- DTCG: https://www.designtokens.org/tr/2025.10/

## Current Implementation

- Use `design-token-tool parse-md <input.md> <output-dir>` to convert DESIGN.md-compatible front matter into a DTCG resolver token directory.
- Conversion only reads the leading `--- ... ---` front matter.
- Current token support includes `colors`, `spacing`, `rounded`, and `typography`.
- The CLI writes this structure under the explicit output directory:
  - `foundation/spacing.tokens.json`
  - `foundation/radius.tokens.json`
  - `foundation/typography.tokens.json`
  - `foundation/semantic-colors.tokens.json`
  - `theme/light.tokens.json`
  - `theme/dark.tokens.json`
  - `tokens.resolver.json`
- Colors ending with `-light` or `-dark` are theme tokens. The suffix is removed in output so both themes override the same token path.
- Colors without a theme suffix are foundation semantic colors.
- Theme colors must have both light and dark pairs. Foundation colors must not conflict with themed colors after suffix removal.
- Color values support `#rrggbb`, `#rrggbbaa`, and aliases like `{colors.xxx}`.
- Dimension values are strict DTCG 2025.10 dimensions: only `px` and `rem` are accepted.
- `spacing` supports numeric DESIGN.md values as DTCG `$type: "number"` tokens.

## Code Layout

- `src/main.rs`: thin binary entry point and top-level error output.
- `src/cli.rs`: CLI argument parsing, subcommand dispatch, and file I/O.
- `src/lib.rs`: library entry point, exports `convert_markdown_to_dtcg`.
- `src/front_matter.rs`: extracts Markdown front matter.
- `src/dtcg.rs`: library entry point for DTCG resolver file generation and output file ordering.
- `src/dtcg/alias.rs`: alias validation and theme alias normalization.
- `src/dtcg/colors.rs`: color splitting between foundation/light/dark and color token JSON generation.
- `src/dtcg/dimension.rs`: dimension and numeric value parsing helpers.
- `src/dtcg/spacing.rs`: spacing and `rounded` to radius token generation.
- `src/dtcg/typography.rs`: typography token generation.
- `src/dtcg/resolver.rs`: `tokens.resolver.json` generation.
- `src/dtcg/tests.rs`: library-level conversion rule tests.
- `src/color.rs`: parses raw color values.
- `tests/cli.rs`: CLI integration snapshot test.
- `tests/fixtures/design.md`: DESIGN.md fixture for tests.
- `tests/snapshots/`: insta snapshots for CLI output.

## Development Guidelines

- Keep core conversion logic in the library; the CLI should only handle I/O.
- When adding token types, extend along the existing `src/dtcg/` module boundaries instead of putting conversion logic into `src/cli.rs` or `src/main.rs`.
- When adding CLI features, prefer new subcommands or subcommand options in `src/cli.rs`.
- Preserve stable JSON output order; the project uses `indexmap` and `serde_json/preserve_order`.
- Error messages should include the relevant token name or field name when possible.
- Only parse front matter. Do not infer tokens from the Markdown body.
- When parsing rules change, update fixtures and insta snapshots together.
- Keep one end-to-end success snapshot in `tests/cli.rs`; avoid duplicating the same full-output snapshot in library unit tests.
- Use library unit tests for focused validation rules and error branches.
- Official snapshots (`*.snap`) should be committed. `*.snap.new` files are temporary insta review files and are ignored.

## Notes

- The repository may be in an early initialization state. Do not delete or reset untracked files unless explicitly asked.
- Generated token files should usually go under `tokens/` in examples or docs, but the CLI writes to the explicit output directory provided by the user.
