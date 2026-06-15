# design-token-tool

`design-token-tool` is a Rust CLI for converting
[DESIGN.md](https://github.com/google-labs-code/design.md/blob/main/docs/spec.md)-compatible
YAML front matter into design token artifacts.

It can generate:

- DTCG 2025.10 token JSON files and a resolver file
- Tailwind CSS v4 theme variables
- Figma Variables import token files from a DTCG resolver directory

The tool only reads the leading Markdown front matter block. Markdown body content
is ignored.

## Status

This project is early-stage and currently supports:

- `colors`
- `spacing`
- `rounded`
- `typography`

Supported color values are `#rrggbb`, `#rrggbbaa`, CSS OKLCH values such as
`oklch(50% 0.1 250deg)`, and aliases such as `{colors.neutral-10}`. Dimension
values follow the DTCG 2025.10 dimension shape and currently accept `px` and
`rem` for DTCG/Tailwind output. Figma export accepts pixel dimensions only and
supports `srgb` and `hsl` color spaces only.

## Installation

Build the CLI from source:

```sh
cargo build --release
```

Run it directly with Cargo:

```sh
cargo run -- md-to-dtcg --input DESIGN.md --output tokens
```

Or use the compiled binary:

```sh
./target/release/design-token-tool md-to-dtcg --input DESIGN.md --output tokens
```

## Input Format

Create a Markdown file with YAML front matter:

```md
---
version: alpha
name: Example UI
description: Design tokens for Example UI.
colors:
  neutral-0: "#ffffff"
  neutral-10: "#fafafa"
  brand: oklch(50% 0.1 250deg)
  background-light: "{colors.neutral-10}"
  background-dark: "#111111"
spacing:
  xs: 4px
  sm: 8px
  columns: 12
rounded:
  sm: 4px
  md: 8px
typography:
  body-md:
    fontFamily: Public Sans
    fontSize: 16px
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: 0px
---

# Design notes

Markdown body content is ignored by the converter.
```

Theme colors use `-light` and `-dark` suffixes. The suffix is removed in generated
theme output, so `background-light` and `background-dark` both become the
`background` token in their respective theme files.

## Commands

### Convert DESIGN.md front matter to DTCG

```sh
design-token-tool md-to-dtcg --input DESIGN.md --output tokens
```

Defaults:

- `--input DESIGN.md`
- `--output tokens`

Output:

```txt
tokens/
  foundation/
    spacing.tokens.json
    radius.tokens.json
    typography.tokens.json
    colors.tokens.json
  theme/
    light.tokens.json
    dark.tokens.json
  tokens.resolver.json
```

### Convert DESIGN.md front matter directly to Tailwind CSS v4

```sh
design-token-tool md-to-tailwind-v4 --input DESIGN.md --output styles
```

Defaults:

- `--input DESIGN.md`
- `--output styles`

Output:

```txt
styles/
  theme.css
```

The generated CSS includes theme variables and dark-mode overrides, but does not
include `@import "tailwindcss"`. Import Tailwind separately in the consuming
application.

Example output shape:

```css
@custom-variant dark (&:where(.dark, .dark *, [data-theme="dark"], [data-theme="dark"] *));

@theme {
  --spacing-xs: 4px;
  --radius-md: 8px;
  --font-body_md: Public Sans;
  --text-body_md: 16px;
  --color-background: var(--color-neutral-10);
}

.dark,
[data-theme="dark"] {
  --color-background: #111111;
}
```

Typography token names replace hyphens with underscores for Tailwind variable
names. For example, `typography.body-md` becomes variables such as
`--font-body_md`, `--text-body_md`, and `--text-body_md--line-height`.

### Convert DTCG resolver output to Tailwind CSS v4

```sh
design-token-tool dtcg-to-tailwind-v4 --resolver tokens/tokens.resolver.json --output tokens
```

Defaults:

- `--resolver tokens/tokens.resolver.json`
- `--output tokens`

The command resolves all resolver `$ref` paths relative to the provided
`tokens.resolver.json` file and writes:

```txt
tokens/
  theme.css
```

### Convert DTCG resolver output to Figma Variables import files

```sh
design-token-tool dtcg-to-figma --resolver tokens/tokens.resolver.json --output figma
```

Defaults:

- `--resolver tokens/tokens.resolver.json`
- `--output figma`

Output:

```txt
figma/
  Foundation/
    default.tokens.json
  Theme/
    light.tokens.json
    dark.tokens.json
```

The Figma export expands resolver collections and modes into Figma-importable
token files. Alias metadata is emitted under `com.figma.aliasData` when an alias
can be resolved.

## Token Rules

Theme color pairs must include both light and dark variants:

```yaml
colors:
  background-light: "#ffffff"
  background-dark: "#111111"
```

Foundation colors must not conflict with themed colors after removing the theme
suffix:

```yaml
colors:
  background: "#ffffff"
  background-light: "#fafafa"
  background-dark: "#111111"
```

The example above is invalid because `background`, `background-light`, and
`background-dark` would all map to the same theme token path.

Spacing accepts both dimensions and plain numeric values:

```yaml
spacing:
  sm: 8px
  columns: 12
```

This produces a DTCG dimension token for `sm` and a number token for `columns`.

## Development

Run the test suite:

```sh
cargo test
```

Review insta snapshots after intentional output changes:

```sh
cargo insta review
```

When conversion behavior changes, update fixtures and snapshots together:

- `tests/fixtures/design.md`
- `tests/snapshots/`

## License

MIT
