## Project Scope

This is a Rust CLI that parses DESIGN.md-compatible YAML front matter from Markdown files, generates DTCG design token JSON plus a DTCG resolver file, and can generate Tailwind CSS v4 theme variables from the DTCG resolver directory.

References:

- DESIGN.md: https://github.com/google-labs-code/design.md/blob/main/docs/spec.md
- DTCG: https://www.designtokens.org/tr/2025.10/

## Current Implementation

- Use `design-token-tool parse-md <input.md> <output-dir>` to convert DESIGN.md-compatible front matter into a DTCG resolver token directory.
- Use `design-token-tool gen-tailwind-v4 <tokens.resolver.json> <output-dir>` to convert a generated DTCG resolver token directory into Tailwind CSS v4 theme variables.
- Conversion only reads the leading `--- ... ---` front matter.
- Current token support includes `colors`, `spacing`, `rounded`, and `typography`.
- The CLI writes this structure under the explicit output directory:
  - `foundation/spacing.tokens.json`
  - `foundation/radius.tokens.json`
  - `foundation/typography.tokens.json`
  - `foundation/colors.tokens.json`
  - `theme/light.tokens.json`
  - `theme/dark.tokens.json`
  - `tokens.resolver.json`
- `gen-tailwind-v4` writes `theme.css` under the explicit output directory.
- `gen-tailwind-v4` resolves all resolver `$ref` paths relative to the provided `tokens.resolver.json` file.
- Tailwind CSS output does not include `@import "tailwindcss"`; it only emits theme variables and dark-mode overrides for use by the consuming application.
- Tailwind CSS output emits light/foundation tokens in `@theme`, emits `@custom-variant dark`, and overrides dark theme tokens under `.dark` and `[data-theme="dark"]`.
- Colors ending with `-light` or `-dark` are theme tokens. The suffix is removed in output so both themes override the same token path.
- Colors without a theme suffix are foundation reference colors and are written to `foundation/colors.tokens.json`.
- Theme colors must have both light and dark pairs. Foundation colors must not conflict with themed colors after suffix removal.
- Color values support `#rrggbb`, `#rrggbbaa`, and aliases like `{colors.xxx}`.
- Dimension values are strict DTCG 2025.10 dimensions: only `px` and `rem` are accepted.
- `spacing` supports numeric DESIGN.md values as DTCG `$type: "number"` tokens.
- Tailwind v4 typography output maps `typography.body-md` style names to `--font-body_md`, `--text-body_md`, and related `--text-body_md--*` variables; hyphens become underscores only for typography token names.

## Code Layout

- `src/main.rs`: thin binary entry point and top-level error output.
- `src/cli.rs`: CLI argument parsing, subcommand dispatch, and file I/O.
- `src/lib.rs`: library entry point, exports `convert_markdown_to_dtcg` and `convert_resolver_to_tailwind_v4`.
- `src/front_matter.rs`: extracts Markdown front matter.
- `src/dtcg.rs`: library entry point for DTCG resolver file generation and output file ordering.
- `src/dtcg/`: DTCG generation modules, split by alias validation, dimension parsing, resolver generation, token groups, and focused conversion rule tests.
- `src/tailwind_v4.rs`: library entry point for Tailwind CSS v4 generation.
- `src/tailwind_v4/`: Tailwind v4 generation modules, split by resolver parsing, token file walking, alias mapping, CSS rendering, value conversion, and token groups.
- `src/color.rs`: parses raw color values.
- `tests/cli.rs`: CLI integration snapshot test.
- `tests/fixtures/design.md`: DESIGN.md fixture for tests.
- `tests/snapshots/`: insta snapshots for CLI output.

## Development Guidelines

- Keep core conversion logic in the library; the CLI should only handle I/O.
- When adding token types, extend along the existing `src/dtcg/` module boundaries instead of putting conversion logic into `src/cli.rs` or `src/main.rs`.
- When adding Tailwind CSS generation behavior, extend along the existing `src/tailwind_v4/` module boundaries instead of putting conversion logic into `src/cli.rs` or `src/main.rs`.
- When adding CLI features, prefer new subcommands or subcommand options in `src/cli.rs`.
- Keep `src/tailwind_v4/` files small and focused; each file should stay around 100 lines or less when practical.
- Preserve stable JSON output order; the project uses `indexmap` and `serde_json/preserve_order`.
- Error messages should include the relevant token name or field name when possible.
- Tailwind generation errors should include the relevant token path or resolver `$ref` when possible.
- Only parse front matter. Do not infer tokens from the Markdown body.
- When parsing rules change, update fixtures and insta snapshots together.
- Keep one end-to-end success snapshot in `tests/cli.rs`; avoid duplicating the same full-output snapshot in library unit tests.
- Use library unit tests for focused validation rules and error branches.
- Official snapshots (`*.snap`) should be committed. `*.snap.new` files are temporary insta review files and are ignored.

## Notes

- The repository may be in an early initialization state. Do not delete or reset untracked files unless explicitly asked.
- Generated token files should usually go under `tokens/` in examples or docs, but the CLI writes to the explicit output directory provided by the user.
