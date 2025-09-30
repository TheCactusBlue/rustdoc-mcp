# 🦀 rustdoc-mcp 🦀

A CLI tool and MCP (Model Context Protocol) server for fetching Rust documentation from docs.rs as Markdown.

## Features

- **CLI Mode**: Fetch and display Rust documentation directly in your terminal
- **MCP Server Mode**: Expose Rust documentation fetching capabilities through the Model Context Protocol
- Supports all Rust item types: crates, modules, structs, enums, traits, functions, and more
- Markdown-formatted output with syntax highlighting

## Installation

```bash
cargo install --path .
```

## Usage

### CLI Mode

Fetch documentation for a specific crate or item:

```bash
# Fetch documentation for a crate
rustdoc-mcp fetch tokio

# Fetch documentation for a specific item with type
rustdoc-mcp fetch atrium_api::agent::Agent --item-type struct

# Fetch documentation for a module
rustdoc-mcp fetch std::collections --item-type module
```

### MCP Server Mode

Run as an MCP server for integration with AI assistants:

```bash
rustdoc-mcp server
```

The server exposes a `fetch_docs` tool that accepts:

- `path`: The crate or item path (e.g., `my_crate::submodule::MyStruct`)
- `item_type`: The type of item (e.g., `struct`, `trait`, `module`, `enum`)

## Configuration

Configure as an MCP server in your Claude Desktop config:

```json
{
  "mcpServers": {
    "rustdoc": {
      "command": "/path/to/rustdoc-mcp",
      "args": ["server"]
    }
  }
}
```

## Dependencies

- `rmcp`: MCP server implementation
- `scraper` & `html2md`/`htmd`: HTML parsing and Markdown conversion
- `reqwest`: HTTP client for fetching documentation
- `clap`: CLI argument parsing
- `termimad`: Terminal Markdown rendering

## License

See LICENSE file for details.
