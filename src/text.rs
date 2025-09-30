//! # rustdoc-text
//!
//! A lightweight library to view Rust documentation as plain text (Markdown).
//!
//! This crate provides both a library and a binary for accessing Rust documentation
//! in plain text format.
//!

use anyhow::{Result, anyhow};
use htmd::HtmlToMarkdown;
use reqwest::Client;
use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use strum_macros::{Display, EnumString};
use tempfile::tempdir;

use crate::item_type::ItemType;

/// Fetches Rust documentation from docs.rs and converts it to Markdown.
///
/// # Arguments
///
/// * `crate_name` - The name of the crate to fetch documentation for
/// * `item_path` - Optional path to a specific item within the crate
///
/// # Returns
///
/// The documentation as Markdown text.
///
/// # Examples
///
/// ```no_run
/// use rustdoc_text::fetch_online_docs;
///
/// # fn main() -> anyhow::Result<()> {
/// let docs = fetch_online_docs("serde", None)?;
/// println!("{}", docs);
/// # Ok(())
/// # }
/// ```
pub async fn fetch_online_docs(
    crate_name: &str,
    module_name: Option<&str>,
    item_path: Option<&str>,
) -> Result<String> {
    let client = Client::new();

    // Construct the URL for docs.rs
    let mut url = format!("https://docs.rs/{}/latest/{}", crate_name, crate_name);
    if let Some(module_name) = module_name {
        url = format!("{}/{}", url, module_name);
    }
    if let Some(path) = item_path {
        url = format!("{}/{}.html", url, path.replace("::", "/"));
    }

    // Fetch the HTML content
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        // dbg!(url);
        return Err(anyhow!(
            "Failed to fetch documentation. Status: {}",
            response.status()
        ));
    }

    let html_content = response.text().await?;
    process_html_content(&html_content)
}

pub async fn rustdoc_fetch(resource: &str, item_type: ItemType) -> Result<String> {
    let is_module = item_type == ItemType::Module;

    let client = Client::new();

    let mut resource: Vec<_> = resource.split("::").collect();
    if !is_module && resource.len() < 1 {
        return Err(anyhow!("The top level resource is always a module"));
    }
    let item = if !is_module { resource.pop() } else { None };

    // Construct the URL for docs.rs
    let url = if let Some(item) = item {
        format!(
            "https://docs.rs/{}/latest/{}/{}.{}.html",
            resource[0],
            resource.join("/"),
            item_type.to_string(),
            item
        )
    } else {
        format!(
            "https://docs.rs/{}/latest/{}/index.html",
            resource[0],
            resource.join("/")
        )
    };

    // if let Some(path) = item_path {
    //     url = format!("{}/{}.html", url, path.replace("::", "/"));
    // }

    // Fetch the HTML content
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        // dbg!(url);
        return Err(anyhow!(
            "Failed to fetch documentation. Status: {}",
            response.status()
        ));
    }

    let html_content = response.text().await?;
    process_html_content(&html_content)
}

/// Process HTML content to extract and convert relevant documentation parts to Markdown.
///
/// # Arguments
///
/// * `html` - The HTML content to process
///
/// # Returns
///
/// The documentation as Markdown text.
pub fn process_html_content(html: &str) -> Result<String> {
    let document = Html::parse_document(html);

    // Select the main content div which contains the documentation
    let main_content_selector = Selector::parse("#main-content").unwrap();
    let main_content = document
        .select(&main_content_selector)
        .next()
        .ok_or_else(|| anyhow!("Could not find main content section"))?;

    // Get HTML content
    let html_content = main_content.inner_html();

    // Convert HTML to Markdown using htmd
    let converter = HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();

    let markdown = converter
        .convert(&html_content)
        .map_err(|e| anyhow!("HTML to Markdown conversion failed: {}", e))?;

    // Clean up the markdown (replace multiple newlines, etc.)
    let cleaned_text = clean_markdown(&markdown);

    Ok(cleaned_text)
}

/// Clean up the markdown output to make it more readable in terminal.
///
/// # Arguments
///
/// * `markdown` - The markdown text to clean
///
/// # Returns
///
/// The cleaned markdown text.
pub fn clean_markdown(markdown: &str) -> String {
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

/// Configuration options for fetching Rust documentation.
pub struct Config {
    /// The name of the crate to fetch documentation for.
    pub crate_name: String,

    /// Optional path to a specific item within the crate.
    pub item_path: Option<String>,

    /// Whether to fetch documentation from docs.rs instead of building locally.
    pub online: bool,
}
