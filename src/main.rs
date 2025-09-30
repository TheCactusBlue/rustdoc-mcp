pub mod item_type;
pub mod server;
pub mod text;

use anyhow::Result;
use clap::Parser;
use rmcp::{ServiceExt, transport};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "rustdoc-mcp")]
#[command(about = "A CLI tool to fetch Rust documentation as Markdown")]
#[command(version)]
struct Cli {
    #[arg(long)]
    /// Run as an MCP server
    server: bool,

    crate_name: Option<String>,
    #[arg(short, long)]
    module: Option<String>,
    /// Optional path to specific item (e.g., "struct::MyStruct")
    #[arg(short, long)]
    item_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.server {
        run_server().await?;
    } else {
        let crate_name = cli.crate_name.ok_or_else(|| {
            anyhow::anyhow!("crate_name is required when not running in server mode")
        })?;

        let docs =
            text::fetch_online_docs(&crate_name, cli.module.as_deref(), cli.item_path.as_deref())
                .await?;
        println!("{}", docs);
    }

    Ok(())
}

async fn run_server() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let server = server::RustdocServer::new();

    let service = server.serve(transport::stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    // Keep the server running indefinitely
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
pub mod test {
    use crate::{item_type::ItemType, text::rustdoc_fetch};

    #[tokio::test]
    pub async fn scratch() {
        // println!(
        //     "{}",
        //     rustdoc_fetch("atrium_api", ItemType::Module).await.unwrap()
        // );
        println!(
            "{}",
            rustdoc_fetch("atrium_api::agent::Agent", ItemType::Struct)
                .await
                .unwrap()
        );
    }
}
