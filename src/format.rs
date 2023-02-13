use tree_sitter::{Node, Parser};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::config::Config;
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

pub fn format_string(contents: &String, mut parser: Parser, config: &Config) -> String {
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
        adapt_indent_level(&node, &mut indent_level, config);

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
        if nesting_level == 1 {
            output.push('\n');
            if !comment_before {
                output.push('\n');
            }
        }
        if node.kind() == "comment" {
            comment_before = true;
        } else {
            comment_before = false;
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
    output.trim().to_owned()
}

pub fn format_file(path: &Path, parser: Parser, config: &Config) {
    let mut contents = String::new();
    println!("File: {}", path.display());
    File::open(path)
        .expect("Unable to open the file")
        .read_to_string(&mut contents)
        .expect("Unable to read the file");
    let output = format_string(&contents, parser, config);
    if get_len(&output) != get_len(&contents) {
        println!(
            "There was an error parsing your code.
Not applying formatting.
Open an issue."
        );
    } else if config.should_preview() {
        println!("{output}");
    } else {
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

fn adapt_indent_level(node: &Node, indent_level: &mut usize, config: &Config) {
    match node.kind() {
        "(" => {
            *indent_level += config.get_indent();
        }
        ")" => {
            *indent_level -= config.get_indent();
        }
        "[" => {
            *indent_level += config.get_list_indent();
        }
        "]" => {
            *indent_level -= config.get_list_indent();
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tree_sitter::Parser;
//
//     #[test]
//     fn list() {
//         let mut parser = Parser::new();
//         parser.set_language(tree_sitter_query::language()).unwrap();
//
//         let input = String::from("[\"(\" \")\" \"[\" \"]\" \"{\" \"}\"]  @punctuation.bracket");
//         assert_eq!(
//             format_string(&input, parser, args),
//             "[
//  \"(\"
//  \")\"
//  \"[\"
//  \"]\"
//  \"{\"
//  \"}\"
// ] @punctuation.bracket"
//         )
//     }
// }
