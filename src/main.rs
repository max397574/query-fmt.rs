use clap::Parser as ClapParser;
use tree_sitter::Parser;

use std::path::Path;

mod args;
mod file_iterator;
mod format;
mod query_tree;

use args::Args;
use file_iterator::RecursiveFileIterator;
use format::format_file;

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.file).to_owned();
    let rec_iterator = RecursiveFileIterator::new(path);
    for file in rec_iterator {
        if let Some(extension) = file.extension() {
            if extension == "scm" {
                let mut parser = Parser::new();
                parser.set_language(tree_sitter_query::language()).unwrap();
                format_file(file.as_path(), parser, &args)
            }
        }
    }
}
