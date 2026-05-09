use indexmap::IndexMap;

const DARK_VARIANT: &str = r#"@custom-variant dark (&:where(.dark, .dark *, [data-theme="dark"], [data-theme="dark"] *));"#;

#[derive(Default)]
pub(super) struct CssVariables {
    values: IndexMap<String, String>,
}

impl CssVariables {
    pub(super) fn insert(&mut self, name: String, value: String) {
        self.values.insert(name, value);
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

pub(super) fn render_css(base: &CssVariables, dark: &CssVariables) -> String {
    let mut css = String::new();
    css.push_str(DARK_VARIANT);
    css.push_str("\n\n@theme {\n");

    for (name, value) in &base.values {
        append_variable(&mut css, name, value);
    }

    css.push('}');

    if !dark.is_empty() {
        css.push_str("\n\n.dark,\n[data-theme=\"dark\"] {\n");

        for (name, value) in &dark.values {
            append_variable(&mut css, name, value);
        }

        css.push('}');
    }

    css.push('\n');
    css
}

fn append_variable(css: &mut String, name: &str, value: &str) {
    css.push_str("  ");
    css.push_str(name);
    css.push_str(": ");
    css.push_str(value);
    css.push_str(";\n");
}
