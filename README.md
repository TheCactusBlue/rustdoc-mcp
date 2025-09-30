# rustdoc-mcp

A CLI tool to fetch Rust documentation as Markdown from docs.rs.

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Usage

Fetch documentation for a crate:

```bash
rustdoc-mcp <crate_name>
```

Example:
```bash
rustdoc-mcp serde
```

### Fetch Module Documentation

Use the `-m` or `--module` flag to fetch documentation for a specific module:

```bash
rustdoc-mcp <crate_name> -m <module_name>
```

Example:
```bash
rustdoc-mcp tokio -m sync
```

### Fetch Item Documentation

Use the `-i` or `--item-path` flag to fetch documentation for a specific item (struct, enum, trait, etc.):

```bash
rustdoc-mcp <crate_name> -i <item_path>
```

Example:
```bash
rustdoc-mcp serde -i "derive.Deserialize"
```

### Combined Options

You can combine module and item path:

```bash
rustdoc-mcp tokio -m sync -i "mpsc.Sender"
```

## Help

```bash
rustdoc-mcp --help
```

## Version

```bash
rustdoc-mcp --version
```
