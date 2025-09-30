pub mod text;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rustdoc-mcp")]
#[command(about = "A CLI tool to fetch Rust documentation as Markdown")]
#[command(version)]
struct Cli {
    crate_name: String,
    #[arg(short, long)]
    module: Option<String>,
    /// Optional path to specific item (e.g., "struct::MyStruct")
    #[arg(short, long)]
    item_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let docs = text::fetch_online_docs(
        &cli.crate_name,
        cli.module.as_deref(),
        cli.item_path.as_deref(),
    )
    .await?;
    println!("{}", docs);

    Ok(())
}
