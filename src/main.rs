use clap::Parser as ClapParser;

use std::path::Path;

mod args;
mod config;
mod file_iterator;
mod format;
mod node_item;
mod query_tree;

use args::Args;
use file_iterator::RecursiveFileIterator;

fn main() {
    let args = Args::parse();
    let config = config::Config::new(&args);

    RecursiveFileIterator::new(Path::new(&args.file).to_owned()).format(&config);
}
