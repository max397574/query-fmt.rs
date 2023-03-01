# query-fmt

### A tool for formatting your tree-sitter queries.

## ✨ Features
### Implemented
- Automatically format single files or directory
- Configurable with cli-flags
- Option to only preview file

## 📦 Installation
### Download release (coming soon)
1. Download query-fmt-{platform}-x86_64.zip
2. Unzip it
3. Run it with query-fmt

### Build from source
1. Install with `cargo install --git https://github.com/max397574/query-fmt.rs.git`
2. run with `query-fmt`

### Build from local directory
1. Clone this directory with `git clone https://github.com/max397574/query-fmt.rs`
2. `cd query-fmt.rs`
3. Build with `cargo build --release`
4. Run with `./target/release/query-fmt`

## 🚀 Usage
```
A formatter for tree-sitter queries

Usage: query-fmt [OPTIONS] <file>

Arguments:
  <file>  Name of the file or directory to format

Options:
  -p, --preview                    Preview the formatted file
      --no-print-filename          Don't print filename in output
  -i, --indent <INDENT>            Indent of nested things [default: 2]
  -l, --list-indent <LIST_INDENT>  Indent of list items [default: 1]
  -h, --help                       Print help
  -V, --version                    Print version
```
