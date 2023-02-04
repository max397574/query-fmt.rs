use clap::Parser as ClapParser;
use std::path::PathBuf;

/// A formatter for tree-sitter queries
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the file to format
    #[arg(value_name = "file")]
    pub file: PathBuf,

    /// If you want to use a custom config file
    #[arg(long)]
    config_file: Option<String>,

    /// Preview the formatted file
    #[arg(short, long)]
    pub preview: bool,
}
