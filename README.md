# query-fmt

### A tool for formatting your tree-sitter queries.

## ✨ Features
### Implemented
- Format `*.scm` files
- Print out formatted file
- Format complete directories

### Planned
- Directly modify file
- Configuration

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
You can use `query-fmt <file>` to format the file.
With the `--preview` flag the file won't be changed.
The formatted version will just be printed out.

You can also use either a `queries` folder or a folder for a specific language inside such a folder (e.g. `queries/lua/`) to format all the files inside the folder.
