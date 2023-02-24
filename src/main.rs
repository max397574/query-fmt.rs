use clap::Parser as ClapParser;

use std::path::Path;

mod args;
mod file_iterator;
mod format;
mod query_tree;
mod config;
mod node_item;

use args::Args;
use file_iterator::RecursiveFileIterator;

fn main() {
    let args=Args::parse();
    let config = config::Config::new(&args);

    RecursiveFileIterator::new(Path::new(&args.file).to_owned()).format(&config);
}
