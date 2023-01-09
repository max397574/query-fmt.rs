use tree_sitter::{Language, Node, Parser};

use std::fs::File;
use std::io::prelude::*;

use clap::Parser as ClapParser;

/// A formatter for tree-sitter queries
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to format
    #[arg(value_name = "file")]
    file: String,

    /// If you want to use a custom config file
    #[arg(long)]
    config_file: Option<String>,
}

fn check_parent(parent_kind: &str, node: Node) -> bool {
    let parent = node.parent();
    if let Some(parent_node) = parent {
        if parent_node.kind() == parent_kind {
            return true;
        }
    }
    false
}

fn main() {
    let args = Args::parse();
    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_query() -> Language;
    }

    let language = unsafe { tree_sitter_query() };
    parser.set_language(language).unwrap();

    let mut file = File::open(args.file).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let source_code = &contents;

    let tree = parser.parse(source_code, None).unwrap();
    let mut comment_before = false;
    let mut output = String::new();
    let mut reached_root = false;
    let mut cursor = tree.walk();
    let mut nesting_level = 0;
    let mut indent_level = 0;
    while !reached_root {
        match cursor.node().kind() {
            "(" => {
                indent_level += 2;
            }
            ")" => {
                indent_level -= 2;
            }
            "[" => {
                indent_level += 1;
            }
            "]" => {
                indent_level -= 1;
            }
            _ => {}
        }
        match cursor.node().kind() {
            "field_definition" => {
                output.push('\n');
                output.push_str(&" ".repeat(indent_level));
            }
            "predicate" => {
                output.push('\n');
                output.push_str(&" ".repeat(indent_level));
            }
            _ => {}
        }
        if cursor.node().kind() == "comment" && !comment_before {
            output.push('\n');
        }
        if cursor.node().kind() == "comment" {
            comment_before = true;
        } else {
            comment_before = false;
        }
        if nesting_level == 1 {
            output.push('\n');
            if cursor.node().kind() != "comment" {
                output.push('\n');
            }
        }
        if cursor.node().kind() == "capture" && !check_parent("parameters", cursor.node()) {
            output.push(' ');
        }

        if cursor.node().kind() == "anonymous_node" && check_parent("list", cursor.node()) {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        if cursor.node().kind() == "]" && check_parent("list", cursor.node()) {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        if cursor.node().kind() == "identifier"
            && check_parent("anonymous_node", cursor.node())
            && !check_parent("list", cursor.node().parent().unwrap())
            && !check_parent("grouping", cursor.node().parent().unwrap())
        {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        if check_parent("parameters", cursor.node()) {
            output.push(' ')
        }

        if check_parent("named_node", cursor.node()) && cursor.node().kind() == "named_node" {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        if cursor.node().child_count() == 0 && cursor.node().kind() != "\""
            || cursor.node().kind() == "string"
        {
            output.push_str(cursor.node().utf8_text(source_code.as_bytes()).unwrap());
        }
        if cursor.node().kind() == "anonymous_node" && check_parent("list", cursor.node()) {
            output.push_str(cursor.node().utf8_text(source_code.as_bytes()).unwrap());
        }
        if cursor.node().kind() == "identifier"
            && check_parent("anonymous_node", cursor.node())
            && !check_parent("list", cursor.node().parent().unwrap())
        {
            output.push_str(cursor.node().utf8_text(source_code.as_bytes()).unwrap());
        }
        if cursor.node().kind() == ":" {
            output.push(' ');
        }
        if cursor.goto_first_child() {
            nesting_level += 1;
            continue;
        }
        if cursor.goto_next_sibling() {
            continue;
        }
        let mut retracing = true;
        while retracing {
            if !cursor.goto_parent() {
                retracing = false;
                reached_root = true;
            } else {
                nesting_level -= 1;
            }
            if cursor.goto_next_sibling() {
                retracing = false;
            }
        }
    }
    println!("{output}");
}
