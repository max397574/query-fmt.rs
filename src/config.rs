use crate::args::Args;

pub struct Config {
    /// Preview the formatted file
    pub preview: bool,

    /// Print filename in output
    pub print_filename: bool,

    pub indent_len: usize,

    pub indent_lists_len: i32,

    pub list_indent: usize,
}

impl Config {
    #[must_use]
    pub const fn new(args: &Args) -> Self {
        Self {
            preview: args.preview,
            indent_len: args.indent_len,
            list_indent: args.list_indent,
            indent_lists_len: args.indent_lists_len,
            print_filename: !args.no_print_filename,
        }
    }
}
