use core::fmt::Write as _;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use tree_sitter::{Node, Parser};

use crate::config::Config;
use crate::node_item::NodeExt;
use crate::query_tree::QueryTree;

macro_rules! add {
    ($this: tt, $($arg: tt)*) => {{
        dbg!("add");
        $this.buffer.write_fmt(format_args!($($arg)*)).unwrap();
    }}
}

macro_rules! add_whitespace {
    ($this: tt) => {{
        for _ in 0..$this.indent_level {
            write!(&mut $this.buffer, " ").unwrap();
        }
    }};
}

macro_rules! add_line {
    ($this: tt, $($arg: tt)*) => {
        dbg!("add_line");
        add_whitespace!($this);
        $this.buffer.write_fmt(format_args!($($arg)*)).unwrap();
        $this.buffer += "\n";
    }
}

macro_rules! indent {
    ($this: tt) => {
        dbg!("indent");
        $this.indent_level += $this.indent_multiplier;
    };
}

macro_rules! dedent {
    ($this: tt) => {
        assert_ne!($this.indent_level, 0);
        $this.indent_level -= $this.indent_multiplier;
    };
}

struct Formatter {
    buffer: String,
    input: String,
    parser: Parser,
    indent_level: usize,
    indent_multiplier: usize,
    min_list_len: usize,
    list_indent_multiplier: i32,
    remove_current_grouping: bool,
}

impl Formatter {
    #[must_use]
    pub fn new(config: &Config, parser: Parser, source_code: String) -> Self {
        Self {
            buffer: String::new(),
            input: source_code,
            parser,
            indent_level: 0,
            indent_multiplier: config.indent_len,
            min_list_len: config.list_indent,
            list_indent_multiplier: config.indent_lists_len,
            remove_current_grouping: false,
        }
    }

    fn adjust_indent_level(&mut self, node: &Node) {
        match node.kind() {
            "(" => {
                indent!(self);
            }
            ")" => {
                dedent!(self);
            }
            "[" => {
                indent!(self);
            }
            "]" if node.parent_eq("list") => {
                dedent!(self);
            }
            _ => {}
        }
    }

    fn indent_list_contents(&mut self, node: &Node) {
        if (node.kind() == "anonymous_node" || node.kind() == "named_node")
            && node.parent_eq("list")
            && node.prev_named_sibling().is_none()
        // only add a newline for the first list item
        {
            add_line!(self, "");
        }
    }

    fn push_text_to_output(&mut self, node: &Node) {
        if node.kind() == "escape_sequence" {
            return;
        }

        let text = node.utf8_text(self.input.as_bytes()).unwrap();
        let is_anon = node.next_named_sibling().map(|node| node.kind()) == Some("anonymous_node");

        // Do not write groupings with an anonymous node, ("foo")
        if text == "(" && is_anon {
            self.remove_current_grouping = true;
        }

        if !self.remove_current_grouping && node.child_count() == 0 && node.kind() != "\""
            || node.kind() == "string"
        {
            add!(self, "{text}");
        }

        // Restore groupings after ")" detected
        if text == ")" && self.remove_current_grouping {
            self.remove_current_grouping = false;
        }

        // Directly add list item text
        if node.kind() == "anonymous_node" && node.parent_eq("list") {
            add_line!(self, "{}", node.utf8_text(self.input.as_bytes()).unwrap());
        }

        if node.kind() == "identifier"
        && node.parent_eq("anonymous_node")
        // Don't add list item text twice
        && !node.grandparent_eq("list")
        {
            add!(self, "{}", node.utf8_text(self.input.as_bytes()).unwrap());
        }
    }

    fn add_spacing_around_parameters(&mut self, node: &Node) {
        if node.parent_eq("parameters") {
            add!(self, " ");
        }
    }

    fn add_space_after_colon(&mut self, node: &Node) {
        if node.kind() == ":" {
            add!(self, " ");
        }
    }

    pub fn format(mut self) -> String {
        let tree = self.parser.parse(&self.input, None).unwrap();
        let mut comment_before = false;
        let mut query_tree = QueryTree {
            cursor: tree.walk(),
            reached_root: false,
            nesting_level: 0,
            first_node: true,
        };

        for node in &mut query_tree {
            // println!("[0]self.buffer={}", self.buffer);
            self.adjust_indent_level(&node.inner);

            if matches!(node.kind(), "comment" | "comment_block") {
                add_line!(self, "");
                println!("1");
                indent!(self);
            }
            if node.kind() == "comment" && !comment_before {
                add_line!(self, "");
            }
            if node.nesting_level == 1 && !node.first_node {
                add_line!(self, "");
                if !comment_before {
                    add_line!(self, "");
                }
            }

            comment_before = node.kind() == "comment";

            if node.kind() == "capture" && !node.parent_eq("parameters") {
                add!(self, " ");
            }

            self.indent_list_contents(&node.inner);

            if node.kind() == "identifier"
                && node.parent_eq("anonymous_node")
                && !node.grandparent_eq("list")
                && !node.grandparent_eq("grouping")
            {
                // TODO: ??
                // add_line!(self, "");
                // indent!(self);
            }

            self.add_spacing_around_parameters(&node.inner);

            if node.parent_eq("named_node")
                && (node.kind() == "named_node" || node.kind() == "list")
            {
                add_line!(self, "");
                println!("4");
                indent!(self);
            }

            self.push_text_to_output(&node.inner);

            self.add_space_after_colon(&node.inner);
        }

        self.buffer
    }
}

pub fn format_file(path: &Path, parser: Parser, config: &Config) {
    if config.print_filename {
        println!("File: {}", path.display());
    }
    let source_code = std::fs::read_to_string(path).expect("Unable to read the file");
    let formatter = Formatter::new(config, parser, source_code);
    let output = formatter.format();
    if config.preview {
        println!("{output}");
    } else {
        let mut new_file = File::create(path).expect("Unable to open the file");
        // writeln!(&mut new_file, "{output}").unwrap();
        new_file
            .write_all(output.as_bytes())
            .expect("Unable to write to the file");
    }
}
