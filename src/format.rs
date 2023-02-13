use tree_sitter::{Node, Parser};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::args::Args;
use crate::query_tree::QueryTree;

fn check_parent(parent_kind: &str, node: &Node) -> bool {
    node.parent()
        .map_or(false, |parent_node| parent_node.kind() == parent_kind)
}

fn get_len(source: &str) -> usize {
    source
        .chars()
        .filter(|char| char != &'\n' && char != &' ' && char != &'\t' && char != &'\r')
        .count()
}

pub fn format_string(contents: &String, mut parser: Parser, args: &Args) -> String {
    let tree = parser.parse(contents, None).unwrap();
    let mut comment_before = false;
    let mut output = String::new();
    let mut query_tree = QueryTree {
        cursor: tree.walk(),
        reached_root: false,
        nesting_level: 0,
    };
    let mut indent_level = 0;
    for (node, nesting_level) in &mut query_tree {
        adapt_indent_level(&node, &mut indent_level, args);

        match node.kind() {
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
        if node.kind() == "comment" && !comment_before {
            output.push('\n');
        }
        if node.kind() == "comment" {
            comment_before = true;
        } else {
            comment_before = false;
        }
        if nesting_level == 1 {
            output.push('\n');
            if node.kind() != "comment" {
                output.push('\n');
            }
        }
        if node.kind() == "capture" && !check_parent("parameters", &node) {
            output.push(' ');
        }

        indent_list_contents(&node, &mut output, indent_level);

        if node.kind() == "]" && check_parent("list", &node) {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        if node.kind() == "identifier"
            && check_parent("anonymous_node", &node)
            && !check_parent("list", &node.parent().unwrap())
            && !check_parent("grouping", &node.parent().unwrap())
        {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        add_spacing_around_parameters(&node, &mut output);

        if check_parent("named_node", &node)
            && (node.kind() == "named_node" || node.kind() == "list")
        {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        push_text_to_output(&node, &mut output, contents);

        add_space_after_colon(&node, &mut output);
    }
    output = output.trim().to_owned();
    output
}

pub fn format_file(path: &Path, parser: Parser, args: &Args) {
    let mut file = File::open(path).expect("Unable to open the file");
    println!("File: {}", path.display());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let source_code = &contents;
    let original_len = get_len(source_code);
    let output = format_string(source_code, parser, args);
    if get_len(&output) != original_len {
        println!(
            "There was an error parsing your code.
Not applying formatting.
Open an issue."
        );
    } else if args.preview {
        println!("{output}");
    } else if !args.preview {
        let mut new_file = File::create(path).expect("Unable to open the file");
        writeln!(&mut new_file, "{output}").unwrap();
    }
}

fn add_spacing_around_parameters(node: &tree_sitter::Node, output: &mut String) {
    if check_parent("parameters", node) {
        output.push(' ')
    }
}

fn push_text_to_output(node: &tree_sitter::Node, output: &mut String, source_code: &String) {
    if node.kind() == "escape_sequence" {
        return;
    }
    if node.child_count() == 0 && node.kind() != "\"" || node.kind() == "string" {
        output.push_str(node.utf8_text(source_code.as_bytes()).unwrap());
    }
    // Directly add list item text
    if node.kind() == "anonymous_node" && check_parent("list", node) {
        output.push_str(node.utf8_text(source_code.as_bytes()).unwrap());
    }
    if node.kind() == "identifier"
        && check_parent("anonymous_node", node)
        // Don't add list item text twice
        && !check_parent("list", &node.parent().unwrap())
    {
        output.push_str(node.utf8_text(source_code.as_bytes()).unwrap());
    }
}

fn add_space_after_colon(node: &tree_sitter::Node, output: &mut String) {
    if node.kind() == ":" {
        output.push(' ');
    }
}

fn adapt_indent_level(node: &Node, indent_level: &mut usize, args: &Args) {
    match node.kind() {
        "(" => {
            *indent_level += args.indent;
        }
        ")" => {
            *indent_level -= args.indent;
        }
        "[" => {
            *indent_level += args.list_indent;
        }
        "]" => {
            *indent_level -= args.list_indent;
        }
        _ => {}
    }
}

fn indent_list_contents(node: &tree_sitter::Node, output: &mut String, indent_level: usize) {
    if (node.kind() == "anonymous_node" || node.kind() == "named_node")
        && check_parent("list", node)
    {
        output.push('\n');
        output.push_str(&" ".repeat(indent_level));
    }
}
