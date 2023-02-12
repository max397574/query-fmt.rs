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

    let path = Path::new(&args.file).to_owned();
    let rec_iterator = RecursiveFileIterator::new(path);
    rec_iterator.format(&args);
}
