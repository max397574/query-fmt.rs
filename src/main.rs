use tree_sitter::{Language, Parser};

fn main() {
    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_query() -> Language;
    }

    let language = unsafe { tree_sitter_query() };
    parser.set_language(language).unwrap();
    let source_code = "(block)@test(mod_item name: (identifier)@namespace)
(scoped_identifier(scoped_identifier path: (identifier) @rust_path) (#set! conceal \"\"))((field_identifier) @constant (#lua-match? @constant \"^[A-Z]\"))
";

    let tree = parser.parse(source_code, None).unwrap();
    // let root_node = tree.root_node();
    let mut output = String::new();
    let mut reached_root = false;
    let mut cursor = tree.walk();
    let mut nesting_level = 0;
    while !reached_root {
        // TODO: space in predicates
        // TODO: newlines
        if nesting_level == 1 {
            output.push_str("\n\n")
        }
        if cursor.node().kind() == "field_definition" {
            output.push_str("\n  ");
        }
        if cursor.node().kind() == "predicate" {
            output.push_str("\n  ");
        }
        if cursor.node().kind() == "capture" {
            output.push(' ');
        }
        let parent = cursor.node().parent();
        if let Some(node) = parent {
            if node.kind() == "parameters" {
                output.push(' ')
            }
        }
        if cursor.node().child_count() == 0 && cursor.node().kind() != "\""
            || cursor.node().kind() == "string"
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
