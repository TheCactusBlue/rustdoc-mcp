//! # rustdoc-search
//!
//! A lightweight library to search for crates on docs.rs.
//!
//! This module provides functionality to search the docs.rs releases index.

use anyhow::{Result, anyhow};
use htmd::{Element, HtmlToMarkdown};
use reqwest::Client;
use scraper::{Html, Selector};

use crate::text::clean_markdown;

pub async fn rustdoc_search(query: &str) -> Result<String> {
    let html_content = rustdoc_search_html(query).await?;
    process_search_html(&html_content)
}

pub async fn rustdoc_search_html(query: &str) -> Result<String> {
    let client = Client::new();

    // Construct the URL for docs.rs search
    let url = format!(
        "https://docs.rs/releases/search?query={}",
        urlencoding::encode(query)
    );

    // Fetch the HTML content
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch search results. Status: {}",
            response.status()
        ));
    }

    let html_content = response.text().await?;
    Ok(html_content)
}

/// Process HTML content to extract and convert search results to Markdown.
///
/// # Arguments
///
/// * `html` - The HTML content to process
///
/// # Returns
///
/// The search results as Markdown text.
pub fn process_search_html(html: &str) -> Result<String> {
    let document = Html::parse_document(html);

    // Select the main content div which contains the search results
    let main_content_selector = Selector::parse(".recent-releases-container > ul").unwrap();
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
