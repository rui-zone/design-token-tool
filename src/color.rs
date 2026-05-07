use serde_json::{Value, json};

#[derive(Debug, PartialEq)]
struct DtcgColor {
    components: [f64; 3],
    alpha: f64,
    hex: String,
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

    if is_valid_alias(raw_value) {
        return Ok(Value::String(raw_value.to_string()));
    }

    Err(format!(
        "invalid color value for `{name}`: expected #rrggbb, #rrggbbaa, or {{path.to.token}}"
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
}
