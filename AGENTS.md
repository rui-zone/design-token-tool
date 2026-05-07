## Project Scope

This is a Rust CLI that parses DESIGN.md-compatible YAML front matter from Markdown files and generates DTCG design token JSON.

References:

- DESIGN.md: https://github.com/google-labs-code/design.md/blob/main/docs/spec.md
- DTCG: https://www.designtokens.org/tr/2025.10/

## Current Implementation

- Use `design-token-tool parse-md <input.md> <output.json>` to convert DESIGN.md-compatible front matter into DTCG JSON.
- Conversion only reads the leading `--- ... ---` front matter.
- Current token support is focused on `colors`: `#rrggbb`, `#rrggbbaa`, and aliases like `{colors.xxx}`.

## Code Layout

- `src/main.rs`: thin binary entry point and top-level error output.
- `src/cli.rs`: CLI argument parsing, subcommand dispatch, and file I/O.
- `src/lib.rs`: library entry point, exports `convert_markdown_to_dtcg`.
- `src/front_matter.rs`: extracts Markdown front matter.
- `src/dtcg.rs`: converts front matter data into DTCG JSON.
- `src/color.rs`: parses color values and aliases.
- `tests/cli.rs`: CLI integration test.
- `tests/fixtures/design.md`: DESIGN.md fixture for tests.

## Development Guidelines

- Keep core conversion logic in the library; the CLI should only handle I/O.
- When adding token types, extend along the existing library module boundaries instead of putting conversion logic into `src/cli.rs` or `src/main.rs`.
- When adding CLI features, prefer new subcommands or subcommand options in `src/cli.rs`.
- Preserve stable JSON output order; the project uses `indexmap` and `serde_json/preserve_order`.
- Error messages should include the relevant token name or field name when possible.
- Only parse front matter. Do not infer tokens from the Markdown body.
- When parsing rules change, update fixtures and tests together.

## Notes

- The repository may be in an early initialization state. Do not delete or reset untracked files unless explicitly asked.
- Generated token files should usually go under `tokens/` in examples or docs, but the current CLI writes to the explicit output path provided by the user.
