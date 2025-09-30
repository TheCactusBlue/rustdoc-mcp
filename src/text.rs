//! # rustdoc-text
//!
//! A lightweight library to view Rust documentation as plain text (Markdown).
//!
//! This crate provides both a library and a binary for accessing Rust documentation
//! in plain text format.
//!

use std::str::FromStr;

use anyhow::{Result, anyhow};
use htmd::{Element, HtmlToMarkdown};
use reqwest::Client;
use scraper::{Html, Selector};

use crate::item_type::ItemType;

pub async fn rustdoc_fetch(
    resource: &str,
    item_type: Option<ItemType>,
    version: Option<&str>,
) -> Result<String> {
    let item_type = if let Some(item_type) = item_type {
        item_type
    } else {
        infer_item_type(resource, version).await?
    };
    let html_content = rustdoc_fetch_html(resource, item_type, version).await?;
    process_html_content(&html_content)
}

pub async fn infer_item_type(resource: &str, version: Option<&str>) -> Result<ItemType> {
    let mut path: Vec<_> = resource.split("::").collect();
    if path.len() <= 1 {
        return Ok(ItemType::Module);
    }
    let _ = path.pop();
    let path = path.join("::");
    let html = rustdoc_fetch_html(&path, ItemType::Module, version).await?;
    let document = Html::parse_document(&html);
    let main_content_selector = Selector::parse("#main-content").unwrap();
    let attrib_selector = Selector::parse(&format!("[title$=\" {}\"]", resource)).unwrap();

    let main_content = document
        .select(&main_content_selector)
        .next()
        .ok_or_else(|| anyhow!("Could not find main content section"))?;

    for el in main_content.select(&attrib_selector) {
        let a = el
            .attr("class")
            .map(|x| ItemType::from_str(x).ok())
            .flatten();
        if let Some(a) = a {
            return Ok(a);
        }
    }

    Err(anyhow!("Could not find the resource"))
}

pub async fn rustdoc_fetch_html(
    resource: &str,
    item_type: ItemType,
    version: Option<&str>,
) -> Result<String> {
    let is_module = item_type == ItemType::Module;

    let client = Client::new();

    let mut resource: Vec<_> = resource.split("::").collect();
    if !is_module && resource.len() < 1 {
        return Err(anyhow!("The top level resource is always a module"));
    }
    let item = if !is_module { resource.pop() } else { None };

    let version_str = version.unwrap_or("latest");

    // Construct the URL for docs.rs
    let url = if let Some(item) = item {
        format!(
            "https://docs.rs/{}/{}/{}/{}.{}.html",
            resource[0],
            version_str,
            resource.join("/"),
            item_type.to_string(),
            item
        )
    } else {
        format!(
            "https://docs.rs/{}/{}/{}/index.html",
            resource[0],
            version_str,
            resource.join("/")
        )
    };

    // if let Some(path) = item_path {
    //     url = format!("{}/{}.html", url, path.replace("::", "/"));
    // }

    // Fetch the HTML content
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch documentation. Status: {}",
            response.status()
        ));
    }

    let html_content = response.text().await?;
    Ok(html_content)
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
        .skip_tags(vec!["script", "style", "button"])
        .add_handler(vec!["dt"], |el: Element| Some(format!("{}:\n", el.content)))
        .add_handler(vec!["dd"], |el: Element| Some(format!("{}\n", el.content)))
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
