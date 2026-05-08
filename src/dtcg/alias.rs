use crate::dtcg::Theme;

pub(super) fn normalize_alias_for_foundation(
    token_name: &str,
    raw_value: &str,
) -> Result<String, String> {
    let Some(reference) = alias_reference(raw_value) else {
        return Ok(raw_value.to_string());
    };

    if reference_has_theme_suffix(reference).is_some() {
        return Err(format!(
            "foundation token `{token_name}` must not reference themed token `{reference}`"
        ));
    }

    Ok(raw_value.to_string())
}

pub(super) fn normalize_alias_for_theme(
    token_name: &str,
    raw_value: &str,
    theme: Theme,
) -> Result<String, String> {
    let Some(reference) = alias_reference(raw_value) else {
        return Ok(raw_value.to_string());
    };

    match reference_has_theme_suffix(reference) {
        Some(alias_theme) if alias_theme == theme => {
            let normalized = reference
                .strip_suffix(theme.suffix())
                .expect("reference suffix should match");
            Ok(format!("{{{normalized}}}"))
        }
        Some(alias_theme) => Err(format!(
            "{theme_label} theme token `{token_name}` must not reference {alias_theme_label} theme token `{reference}`",
            theme_label = theme.label(),
            alias_theme_label = alias_theme.label()
        )),
        None => Ok(raw_value.to_string()),
    }
}

pub(super) fn is_alias(raw_value: &str) -> bool {
    let Some(reference) = alias_reference(raw_value) else {
        return false;
    };

    !reference.is_empty()
        && !reference.contains('{')
        && !reference.contains('}')
        && reference
            .split('.')
            .all(|segment| !segment.is_empty() && !segment.starts_with('$'))
}

fn alias_reference(raw_value: &str) -> Option<&str> {
    raw_value
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
}

fn reference_has_theme_suffix(reference: &str) -> Option<Theme> {
    let last_segment = reference.rsplit('.').next().unwrap_or(reference);

    if last_segment.ends_with(Theme::Light.suffix()) {
        Some(Theme::Light)
    } else if last_segment.ends_with(Theme::Dark.suffix()) {
        Some(Theme::Dark)
    } else {
        None
    }
}
