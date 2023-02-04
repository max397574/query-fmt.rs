use std::fs::read_dir;
use std::fs::DirEntry;

pub struct RecursiveFileIterator {
    pub stack: Vec<DirEntry>,
}

impl Iterator for RecursiveFileIterator {
    type Item = DirEntry;
    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.stack.pop()?;
        if entry.metadata().ok()?.is_dir() {
            for sub_entry in read_dir(entry.path()).ok()?.flatten() {
                self.stack.push(sub_entry);
            }
            return Some(entry);
        }
        None
    }
}
