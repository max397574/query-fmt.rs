use clap::Parser as ClapParser;

use std::path::Path;

mod args;
mod file_iterator;
mod format;
mod query_tree;

use args::Args;
use file_iterator::RecursiveFileIterator;

fn main() {
    let args = Args::parse();

    RecursiveFileIterator::new(Path::new(&args.file).to_owned()).format(&args);
}
