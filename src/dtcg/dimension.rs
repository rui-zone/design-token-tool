use serde_json::{Number, Value, json};

pub(super) fn parse_dimension(name: &str, raw_value: &str) -> Result<Value, String> {
    let unit_start = raw_value
        .find(|character: char| character.is_ascii_alphabetic())
        .ok_or_else(|| {
            format!("invalid dimension for `{name}`: expected a number followed by px or rem")
        })?;
    let (value, unit) = raw_value.split_at(unit_start);

    if unit != "px" && unit != "rem" {
        return Err(format!(
            "invalid dimension for `{name}`: DTCG 2025.10 supports only px and rem"
        ));
    }

    let numeric_value = value
        .parse::<f64>()
        .map_err(|error| format!("invalid dimension for `{name}`: {error}"))?;

    Ok(json!({
        "value": numeric_value,
        "unit": unit,
    }))
}

pub(super) fn yaml_number_to_json_value(
    name: &str,
    number: &serde_yaml::Number,
) -> Result<Value, String> {
    if let Some(value) = number.as_i64() {
        return Ok(Value::Number(Number::from(value)));
    }

    if let Some(value) = number.as_u64() {
        return Ok(Value::Number(Number::from(value)));
    }

    let value = number
        .as_f64()
        .ok_or_else(|| format!("invalid number for `{name}`"))?;
    let number = Number::from_f64(value).ok_or_else(|| format!("invalid number for `{name}`"))?;
    Ok(Value::Number(number))
}
