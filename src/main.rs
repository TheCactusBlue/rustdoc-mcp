pub mod item_type;
pub mod server;
pub mod text;

use anyhow::Result;
use clap::{Parser, Subcommand};
use rmcp::{ServiceExt, transport};
use termimad::{
    MadSkin,
    crossterm::style::{Attribute, Color},
};
use tracing_subscriber::EnvFilter;

use crate::item_type::ItemType;

#[derive(Parser)]
#[command(name = "rustdoc-mcp")]
#[command(about = "A CLI tool to fetch Rust documentation as Markdown")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as an MCP server
    Server,
    /// Fetch documentation for a crate or item
    Fetch {
        /// The crate or item path to fetch documentation for
        resource: String,
        /// Optional type of item to fetch (e.g., struct, trait, module)
        #[arg(short, long)]
        item_type: Option<ItemType>,
        /// Optional version of the crate (defaults to latest)
        #[arg(short, long)]
        version: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server => {
            run_server().await?;
        }
        Commands::Fetch {
            resource,
            item_type,
            version,
        } => {
            let docs = text::rustdoc_fetch(&resource, item_type, version.as_deref()).await?;
            let mut skin = MadSkin::default();

            for h in &mut skin.headers {
                h.add_attr(Attribute::Bold);
                h.set_fg(Color::AnsiValue(172));
            }

            skin.print_text(&docs);
            // println!("{}", docs);
        }
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
            rustdoc_fetch("atrium_api::agent::Agent", Some(ItemType::Struct), None)
                .await
                .unwrap()
        );
    }
}
