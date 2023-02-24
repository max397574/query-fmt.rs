use crate::args::Args;

pub struct Config {
    /// Preview the formatted file
    preview: bool,

    /// Print filename in output
    print_filename: bool,

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
            print_filename: !args.no_print_filename,
        }
    }

    pub fn should_preview(&self) -> bool {
        self.preview
    }

    pub fn should_print_filename(&self) -> bool {
        self.print_filename
    }

    pub fn indent(&self) -> usize {
        self.indent
    }

    pub fn list_indent(&self) -> usize {
        self.list_indent
    }
}
