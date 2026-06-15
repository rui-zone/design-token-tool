use serde_json::{Value, json};

#[derive(Debug, PartialEq)]
struct DtcgColor {
    components: [f64; 3],
    alpha: f64,
    hex: String,
}

#[derive(Debug, PartialEq)]
struct OklchColor {
    components: [f64; 3],
    alpha: f64,
}

/// Parses a color token value into the DTCG JSON representation.
pub fn parse_color_token_value(name: &str, raw_value: &str) -> Result<Value, String> {
    if raw_value.starts_with('#') {
        let color = parse_hex_color(raw_value)
            .map_err(|error| format!("invalid hex color for `{name}`: {error}"))?;

        return Ok(json!({
            "colorSpace": "srgb",
            "components": color.components,
            "alpha": color.alpha,
            "hex": color.hex,
        }));
    }

    if raw_value.to_ascii_lowercase().starts_with("oklch(") {
        let color = parse_oklch_color(raw_value)
            .map_err(|error| format!("invalid oklch color for `{name}`: {error}"))?;

        return Ok(json!({
            "colorSpace": "oklch",
            "components": color.components,
            "alpha": color.alpha,
        }));
    }

    if is_valid_alias(raw_value) {
        return Ok(Value::String(raw_value.to_string()));
    }

    Err(format!(
        "invalid color value for `{name}`: expected #rrggbb, #rrggbbaa, oklch(...), or {{path.to.token}}"
    ))
}

/// Parses a six- or eight-digit hexadecimal color into normalized sRGB channels.
fn parse_hex_color(input: &str) -> Result<DtcgColor, String> {
    let hex = input
        .strip_prefix('#')
        .ok_or_else(|| "value must start with `#`".to_string())?;

    if hex.len() != 6 && hex.len() != 8 {
        return Err("expected 6 or 8 hexadecimal digits after `#`".to_string());
    }

    if !hex.chars().all(|character| character.is_ascii_hexdigit()) {
        return Err("contains non-hexadecimal characters".to_string());
    }

    let red = parse_hex_byte(&hex[0..2])?;
    let green = parse_hex_byte(&hex[2..4])?;
    let blue = parse_hex_byte(&hex[4..6])?;
    let alpha = if hex.len() == 8 {
        parse_hex_byte(&hex[6..8])? as f64 / 255.0
    } else {
        1.0
    };

    Ok(DtcgColor {
        components: [
            round_to_two_decimals(red as f64 / 255.0),
            round_to_two_decimals(green as f64 / 255.0),
            round_to_two_decimals(blue as f64 / 255.0),
        ],
        alpha: round_to_two_decimals(alpha),
        hex: format!("#{}", hex[0..6].to_ascii_lowercase()),
    })
}

/// Parses a CSS OKLCH color into normalized components.
fn parse_oklch_color(input: &str) -> Result<OklchColor, String> {
    let normalized = input.to_ascii_lowercase();
    let inner = normalized
        .strip_prefix("oklch(")
        .and_then(|value| value.strip_suffix(')'))
        .ok_or_else(|| "expected oklch(...)".to_string())?;

    let (components_part, alpha_part) = match inner.split_once('/') {
        Some((components, alpha)) => (components, Some(alpha)),
        None => (inner, None),
    };

    let tokens: Vec<&str> = components_part.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(format!("expected 3 components, got {}", tokens.len()));
    }

    let lightness = parse_oklch_lightness(tokens[0])?;
    let chroma = parse_oklch_chroma(tokens[1])?;
    let hue = parse_oklch_hue(tokens[2])?;
    let alpha = parse_oklch_alpha(alpha_part.unwrap_or("1"))?;

    Ok(OklchColor {
        components: [lightness, chroma, hue],
        alpha,
    })
}

fn parse_oklch_lightness(input: &str) -> Result<f64, String> {
    if let Some(value) = input.strip_suffix('%') {
        let percentage = parse_finite_number(value)
            .map_err(|_| format!("invalid lightness percentage `{input}`"))?;
        if !(0.0..=100.0).contains(&percentage) {
            return Err(format!(
                "lightness percentage `{input}` out of range [0%, 100%]"
            ));
        }
        return Ok(percentage / 100.0);
    }

    let value = parse_finite_number(input).map_err(|_| format!("invalid lightness `{input}`"))?;
    if !(0.0..=1.0).contains(&value) {
        return Err(format!("lightness `{input}` out of range [0, 1]"));
    }
    Ok(value)
}

fn parse_oklch_chroma(input: &str) -> Result<f64, String> {
    let value = parse_finite_number(input).map_err(|_| format!("invalid chroma `{input}`"))?;
    if value < 0.0 {
        return Err(format!("chroma `{input}` must be non-negative"));
    }
    Ok(value)
}

fn parse_oklch_hue(input: &str) -> Result<f64, String> {
    let (value_str, multiplier) = if let Some(value) = input.strip_suffix("deg") {
        (value, 1.0)
    } else if let Some(value) = input.strip_suffix("grad") {
        (value, 0.9)
    } else if let Some(value) = input.strip_suffix("rad") {
        (value, 180.0 / std::f64::consts::PI)
    } else if let Some(value) = input.strip_suffix("turn") {
        (value, 360.0)
    } else {
        (input, 1.0)
    };

    let value = parse_finite_number(value_str).map_err(|_| format!("invalid hue `{input}`"))?;
    Ok(value * multiplier)
}

fn parse_oklch_alpha(input: &str) -> Result<f64, String> {
    let trimmed = input.trim();
    if let Some(value) = trimmed.strip_suffix('%') {
        let percentage = parse_finite_number(value)
            .map_err(|_| format!("invalid alpha percentage `{trimmed}`"))?;
        if !(0.0..=100.0).contains(&percentage) {
            return Err(format!(
                "alpha percentage `{trimmed}` out of range [0%, 100%]"
            ));
        }
        return Ok(percentage / 100.0);
    }

    let value = parse_finite_number(trimmed).map_err(|_| format!("invalid alpha `{trimmed}`"))?;
    if !(0.0..=1.0).contains(&value) {
        return Err(format!("alpha `{trimmed}` out of range [0, 1]"));
    }
    Ok(value)
}

fn parse_finite_number(input: &str) -> Result<f64, String> {
    let value = input.parse::<f64>().map_err(|error| error.to_string())?;
    if !value.is_finite() {
        return Err("value must be finite".to_string());
    }
    Ok(value)
}

/// Parses a two-character hexadecimal channel into a byte.
fn parse_hex_byte(input: &str) -> Result<u8, String> {
    u8::from_str_radix(input, 16)
        .map_err(|error| format!("failed to parse hex byte `{input}`: {error}"))
}

/// Rounds a floating-point value to two decimal places.
fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// Checks whether a raw token value is a simple DTCG alias reference.
fn is_valid_alias(input: &str) -> bool {
    let Some(reference) = input
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
    else {
        return false;
    };

    !reference.is_empty()
        && !reference.contains('{')
        && !reference.contains('}')
        && reference
            .split('.')
            .all(|segment| !segment.is_empty() && !segment.starts_with('$'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_six_digit_hex_color() {
        let color = parse_hex_color("#ffffff").expect("hex color should parse");

        assert_eq!(
            color,
            DtcgColor {
                components: [1.0, 1.0, 1.0],
                alpha: 1.0,
                hex: "#ffffff".to_string(),
            }
        );
    }

    #[test]
    fn parses_eight_digit_hex_color_with_alpha() {
        let color = parse_hex_color("#00000005").expect("hex color should parse");

        assert_eq!(
            color,
            DtcgColor {
                components: [0.0, 0.0, 0.0],
                alpha: 0.02,
                hex: "#000000".to_string(),
            }
        );
    }

    #[test]
    fn rounds_channels_to_two_decimals() {
        let color = parse_hex_color("#fafafa0a").expect("hex color should parse");

        assert_eq!(
            color,
            DtcgColor {
                components: [0.98, 0.98, 0.98],
                alpha: 0.04,
                hex: "#fafafa".to_string(),
            }
        );
    }

    #[test]
    fn rejects_hex_colors_with_invalid_length() {
        let error = parse_hex_color("#fff").expect_err("short hex should be rejected");

        assert!(error.contains("expected 6 or 8"));
    }

    #[test]
    fn rejects_hex_colors_with_invalid_characters() {
        let error = parse_hex_color("#fffffg").expect_err("invalid hex should be rejected");

        assert!(error.contains("non-hexadecimal"));
    }

    #[test]
    fn parses_oklch_with_percentage_lightness() {
        let value = parse_color_token_value("brand", "oklch(50% 0.1 250deg)")
            .expect("oklch color should parse");

        assert_eq!(
            value,
            json!({
                "colorSpace": "oklch",
                "components": [0.5, 0.1, 250.0],
                "alpha": 1.0,
            })
        );
    }

    #[test]
    fn parses_oklch_with_number_lightness() {
        let value = parse_color_token_value("brand", "oklch(0.5 0.1 250)")
            .expect("oklch color should parse");

        assert_eq!(
            value,
            json!({
                "colorSpace": "oklch",
                "components": [0.5, 0.1, 250.0],
                "alpha": 1.0,
            })
        );
    }

    #[test]
    fn parses_oklch_with_alpha() {
        let value = parse_color_token_value("brand", "oklch(50% 0.1 250deg / 0.5)")
            .expect("oklch color should parse");

        assert_eq!(
            value,
            json!({
                "colorSpace": "oklch",
                "components": [0.5, 0.1, 250.0],
                "alpha": 0.5,
            })
        );
    }

    #[test]
    fn parses_oklch_with_alpha_percentage() {
        let value = parse_color_token_value("brand", "oklch(0.5 0.1 250 / 50%)")
            .expect("oklch color should parse");

        assert_eq!(
            value,
            json!({
                "colorSpace": "oklch",
                "components": [0.5, 0.1, 250.0],
                "alpha": 0.5,
            })
        );
    }

    #[test]
    fn preserves_oklch_precision() {
        let value = parse_color_token_value("brand", "oklch(50.123% 0.004 250.567deg / 0.333)")
            .expect("precise oklch color should parse");

        assert_eq!(
            value,
            json!({
                "colorSpace": "oklch",
                "components": [0.50123, 0.004, 250.567],
                "alpha": 0.333,
            })
        );
    }

    #[test]
    fn parses_oklch_hue_units() {
        let degrees = parse_oklch_hue("250deg").expect("deg should parse");
        assert_eq!(degrees, 250.0);

        let radians = parse_oklch_hue("1rad").expect("rad should parse");
        assert!((radians - 57.2958).abs() < 0.001);

        let gradians = parse_oklch_hue("100grad").expect("grad should parse");
        assert_eq!(gradians, 90.0);

        let turns = parse_oklch_hue("0.5turn").expect("turn should parse");
        assert_eq!(turns, 180.0);
    }

    #[test]
    fn rejects_oklch_with_too_few_components() {
        let error = parse_color_token_value("brand", "oklch(50% 0.1)")
            .expect_err("short oklch should fail");

        assert!(error.contains("expected 3 components"));
    }

    #[test]
    fn rejects_oklch_with_invalid_lightness() {
        let error = parse_color_token_value("brand", "oklch(150% 0.1 250deg)")
            .expect_err("out of range lightness should fail");

        assert!(error.contains("lightness"));
    }

    #[test]
    fn rejects_oklch_with_negative_chroma() {
        let error = parse_color_token_value("brand", "oklch(50% -0.1 250deg)")
            .expect_err("negative chroma should fail");

        assert!(error.contains("chroma"));
    }

    #[test]
    fn rejects_oklch_with_non_finite_components() {
        let chroma_error = parse_color_token_value("brand", "oklch(50% nan 250deg)")
            .expect_err("NaN chroma should fail");
        assert!(chroma_error.contains("chroma"));

        let hue_error = parse_color_token_value("brand", "oklch(50% 0.1 inf)")
            .expect_err("infinite hue should fail");
        assert!(hue_error.contains("hue"));
    }

    #[test]
    fn preserves_oklch_case_in_input() {
        let value = parse_color_token_value("brand", "OKLCH(50% 0.1 250DEG)")
            .expect("uppercase oklch should parse");

        assert_eq!(value["colorSpace"], "oklch");
    }
}
