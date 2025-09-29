use html2md::parse_html;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocRequest {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Documentation {
    pub docs: String,
    pub reexports: Vec<String>,
}

impl Documentation {
    pub fn from_string(html_str: &str) -> Documentation {
        let docblock_sel = Selector::parse("div.docblock").unwrap();
        let reexports_sel = Selector::parse("dt[id^=\"reexport\"]").unwrap();

        let html = Html::parse_document(html_str);

        let docs: Vec<_> = html
            .select(&docblock_sel)
            .map(|el| parse_html(&el.inner_html()))
            .collect();

        let reexports: Vec<_> = html
            .select(&reexports_sel)
            .map(|x| parse_html(&x.inner_html()))
            .collect();
        dbg!(reexports);
        Documentation {
            docs: docs.join("\n\n"),
            reexports: html
                .select(&reexports_sel)
                .map(|x| parse_html(&x.inner_html()))
                .collect(),
        }
    }
    pub fn pretty_print(&self) {
        println!("{}", self.docs);
        println!("");
        println!("## Re-exports\n");
        for x in &self.reexports {
            println!("{}", x)
        }
        println!("");
    }
}

#[tokio::main]
async fn main() {
    let docs = reqwest::get("https://docs.rs/rmcp/latest/rmcp/")
        .await
        .unwrap();
    let docs = Documentation::from_string(&docs.text().await.unwrap());
    docs.pretty_print();
}
