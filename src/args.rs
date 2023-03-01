use clap::Parser as ClapParser;
use std::path::PathBuf;

/// A formatter for tree-sitter queries
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the file or directory to format
    #[arg(value_name = "file")]
    pub file: PathBuf,

    /// Preview the formatted file
    #[arg(short, long, default_value_t = false)]
    pub preview: bool,

    /// Don't print filename in output
    #[arg(long, default_value_t = false)]
    pub no_print_filename: bool,

    /// Indent of nested things
    #[arg(short, long, default_value_t = 2)]
    pub indent: usize,

    /// Indent of list items
    #[arg(short, long, default_value_t = 1)]
    pub list_indent: usize,
}
