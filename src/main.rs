use clap::Parser as ClapParser;
use tree_sitter::Parser;

use std::fs::read_dir;
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

    let mut parser = Parser::new();

    parser.set_language(tree_sitter_query::language()).unwrap();

    let path = Path::new(&args.file).to_owned();
    if path.is_file() {
        format_file(&path, parser, &args);
    } else if path.is_dir() {
        let mut stack = Vec::new();
        for subdir in read_dir(path).unwrap().flatten() {
            stack.push(subdir);
        }
        let rec_iterator = RecursiveFileIterator { stack };
        for file in rec_iterator {
            let path = file.path();
            if let Some(extension) = path.extension() {
                if extension == "scm" {
                    let mut parser = Parser::new();
                    parser.set_language(tree_sitter_query::language()).unwrap();
                    format_file(path.as_path(), parser, &args)
                }
            }
        }
    }
}
