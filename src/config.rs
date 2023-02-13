use crate::args::Args;

pub struct Config {
    /// Preview the formatted file
    preview: bool,

    /// Indent of nested things
    indent: usize,

    /// Indent of list items
    list_indent: usize,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        Self {
            preview: args.preview,
            indent: args.indent,
            list_indent: args.list_indent,
        }
    }

    pub fn should_preview(&self) -> bool {
        self.preview
    }

    pub fn get_indent(&self) -> usize {
        self.indent
    }

    pub fn get_list_indent(&self) -> usize {
        self.list_indent
    }
}
