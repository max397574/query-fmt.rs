use std::fs::read_dir;
use std::path::PathBuf;

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
