use anyhow::{Result, anyhow};
use htmd::{Element, HtmlToMarkdown};

fn clean_markdown(markdown: &str) -> String {
    // Replace 3+ consecutive newlines with 2 newlines
    let mut result = String::new();
    let mut last_was_newline = false;
    let mut newline_count = 0;

    for c in markdown.chars() {
        if c == '\n' {
            newline_count += 1;
            if newline_count <= 2 {
                result.push(c);
            }
            last_was_newline = true;
        } else {
            if last_was_newline {
                newline_count = 0;
                last_was_newline = false;
            }
            result.push(c);
        }
    }

    result
}

pub fn html_to_md(input: &str) -> Result<String> {
    // Convert HTML to Markdown using htmd
    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "button"])
        .add_handler(vec!["dt"], |el: Element| Some(format!("{}:\n", el.content)))
        .add_handler(vec!["dd"], |el: Element| Some(format!("{}\n", el.content)))
        .build();

    let markdown = converter
        .convert(&input)
        .map_err(|e| anyhow!("HTML to Markdown conversion failed: {}", e))?;

    // Clean up the markdown (replace multiple newlines, etc.)
    Ok(clean_markdown(&markdown))
}
