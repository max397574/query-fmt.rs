use std::fs::read_dir;
use std::path::PathBuf;
use tree_sitter::Parser;

use crate::config::Config;
use crate::format::format_file;

pub struct RecursiveFileIterator {
    pub stack: Vec<PathBuf>,
}

impl RecursiveFileIterator {
    pub fn new(path: PathBuf) -> Self {
        let mut stack = Vec::new();
        if path.is_file() {
            stack.push(path);
        } else if path.is_dir() {
            for subdir in read_dir(path).unwrap().flatten() {
                stack.push(subdir.path());
            }
        }
        Self { stack }
    }
    pub fn format(self, config: &Config) {
        for file in self {
            if let Some(extension) = file.extension() {
                if extension == "scm" {
                    let mut parser = Parser::new();
                    parser.set_language(tree_sitter_query::language()).unwrap();
                    format_file(file.as_path(), parser, config)
                }
            }
        }
    }
}

impl Iterator for RecursiveFileIterator {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.stack.pop()?;
        if entry.metadata().ok()?.is_dir() {
            for sub_entry in read_dir(entry.clone()).ok()?.flatten() {
                self.stack.push(sub_entry.path());
            }
            return Some(entry);
        } else if entry.is_file() {
            return Some(entry);
        }
        None
    }
}
