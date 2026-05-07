/// Extracts the YAML front matter block from the beginning of a Markdown document.
pub fn extract_front_matter(markdown: &str) -> Result<&str, String> {
    let mut lines = markdown.lines();
    let Some(first_line) = lines.next() else {
        return Err("missing YAML front matter: file is empty".to_string());
    };

    if first_line.trim() != "---" {
        return Err("missing YAML front matter: file must start with `---`".to_string());
    }

    let content_start = first_line.len() + markdown[first_line.len()..].find('\n').unwrap_or(0);
    let content_start = if markdown.as_bytes().get(content_start) == Some(&b'\n') {
        content_start + 1
    } else {
        first_line.len()
    };

    for (offset, line) in markdown[content_start..].lines().enumerate() {
        if line.trim() == "---" {
            let end = byte_offset_for_line(&markdown[content_start..], offset);
            return Ok(&markdown[content_start..content_start + end]);
        }
    }

    Err("missing YAML front matter closing `---`".to_string())
}

/// Returns the byte offset for the start of a line within a UTF-8 string.
fn byte_offset_for_line(text: &str, line_index: usize) -> usize {
    text.lines()
        .take(line_index)
        .map(|line| line.len() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_front_matter() {
        let markdown = "---\ncolors:\n  white: \"#ffffff\"\n---\n# Docs\n";
        let front_matter = extract_front_matter(markdown).expect("front matter should parse");

        assert_eq!(front_matter, "colors:\n  white: \"#ffffff\"\n");
    }

    #[test]
    fn rejects_front_matter_without_closing_marker() {
        let error =
            extract_front_matter("---\ncolors:\n  white: \"#ffffff\"\n").expect_err("should fail");

        assert!(error.contains("closing"));
    }
}
