use tree_sitter::{Language, Node, Parser};

use std::fs::{read_dir, DirEntry, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use clap::Parser as ClapParser;

/// A formatter for tree-sitter queries
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to format
    #[arg(value_name = "file")]
    file: PathBuf,

    /// If you want to use a custom config file
    #[arg(long)]
    config_file: Option<String>,

    /// Preview the formatted file
    #[arg(short, long)]
    preview: bool,
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

fn get_len(source: &str) -> usize {
    source
        .chars()
        .filter(|char| char != &'\n' && char != &' ' && char != &'\t' && char != &'\r')
        .collect::<Vec<char>>()
        .len()
}

struct RecursiveFileIterator {
    stack: Vec<DirEntry>,
}

impl Iterator for RecursiveFileIterator {
    type Item = DirEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.stack.pop() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    if let Ok(dir) = read_dir(entry.path()) {
                        for sub_entry in dir {
                            if let Ok(sub_entry) = sub_entry {
                                self.stack.push(sub_entry);
                            }
                        }
                    }
                }
                return Some(entry);
            }
        }
        None
    }
}

fn main() {
    let args = Args::parse();

    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_query() -> Language;
    }

    let language = unsafe { tree_sitter_query() };
    parser.set_language(language).unwrap();

    let path = Path::new(&args.file).to_owned();
    if path.is_file() {
        format_file(&path, parser, &args);
    } else if path.is_dir() {
        let mut stack = Vec::new();
        for subdir in read_dir(path).unwrap() {
            if let Ok(subdir) = subdir {
                stack.push(subdir);
            }
        }
        let rec_iterator = RecursiveFileIterator { stack };
        for file in rec_iterator {
            let path = file.path();
            if let Some(extension) = path.extension() {
                if extension == "scm" {
                    let mut parser = Parser::new();
                    let language = unsafe { tree_sitter_query() };
                    parser.set_language(language).unwrap();
                    format_file(path.as_path(), parser, &args)
                }
            }
        }
    }
}

fn format_file(path: &Path, mut parser: Parser, args: &Args) {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let source_code = &contents;
    let original_len = get_len(source_code);
    let tree = parser.parse(source_code, None).unwrap();
    let mut comment_before = false;
    let mut output = String::new();
    let mut reached_root = false;
    let mut cursor = tree.walk();
    let mut nesting_level = 0;
    let mut indent_level = 0;
    while !reached_root {
        adapt_indent_level(&cursor, &mut indent_level);

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

        indent_list_contents(&cursor, &mut output, indent_level);

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

        add_spacing_around_parameters(&cursor, &mut output);

        if check_parent("named_node", cursor.node()) && cursor.node().kind() == "named_node" {
            output.push('\n');
            output.push_str(&" ".repeat(indent_level));
        }

        push_text_to_output(&cursor, &mut output, source_code);

        add_space_after_colon(&cursor, &mut output);

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
    output = output.trim().to_owned();
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

fn add_spacing_around_parameters(cursor: &tree_sitter::TreeCursor, output: &mut String) {
    if check_parent("parameters", cursor.node()) {
        output.push(' ')
    }
}

fn push_text_to_output(
    cursor: &tree_sitter::TreeCursor,
    output: &mut String,
    source_code: &String,
) {
    if cursor.node().kind() == "escape_sequence" {
        return;
    }
    if cursor.node().child_count() == 0 && cursor.node().kind() != "\""
        || cursor.node().kind() == "string"
    {
        output.push_str(cursor.node().utf8_text(source_code.as_bytes()).unwrap());
    }
    // Directly add list item text
    if cursor.node().kind() == "anonymous_node" && check_parent("list", cursor.node()) {
        output.push_str(cursor.node().utf8_text(source_code.as_bytes()).unwrap());
    }
    if cursor.node().kind() == "identifier"
        && check_parent("anonymous_node", cursor.node())
        // Don't add list item text twice
        && !check_parent("list", cursor.node().parent().unwrap())
    {
        output.push_str(cursor.node().utf8_text(source_code.as_bytes()).unwrap());
    }
}

fn add_space_after_colon(cursor: &tree_sitter::TreeCursor, output: &mut String) {
    if cursor.node().kind() == ":" {
        output.push(' ');
    }
}

fn adapt_indent_level(cursor: &tree_sitter::TreeCursor, indent_level: &mut usize) {
    match cursor.node().kind() {
        "(" => {
            *indent_level += 2;
        }
        ")" => {
            *indent_level -= 2;
        }
        "[" => {
            *indent_level += 1;
        }
        "]" => {
            *indent_level -= 1;
        }
        _ => {}
    }
}

fn indent_list_contents(
    cursor: &tree_sitter::TreeCursor,
    output: &mut String,
    indent_level: usize,
) {
    if (cursor.node().kind() == "anonymous_node" || cursor.node().kind() == "named_node")
        && check_parent("list", cursor.node())
    {
        output.push('\n');
        output.push_str(&" ".repeat(indent_level));
    }
}
