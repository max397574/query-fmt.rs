use tree_sitter::{Language, Node, Parser};

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
    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_query() -> Language;
    }

    let language = unsafe { tree_sitter_query() };
    parser.set_language(language).unwrap();
    let source_code = "(block)@test(mod_item name: (identifier)@namespace)
(scoped_identifier(scoped_identifier path: (identifier) @rust_path) (#set! conceal \"\"))((field_identifier) @constant (#lua-match? @constant \"^[A-Z]\"))
\"=\" @something
(\"=\") @something
(helloworld
  \"hello\"
  (mynode)
  \"world\")
[\"(\" \")\" \"[\" \"]\" \"{\" \"}\"]  @punctuation.bracket";

    let tree = parser.parse(source_code, None).unwrap();
    // let root_node = tree.root_node();
    let mut output = String::new();
    let mut reached_root = false;
    let mut cursor = tree.walk();
    let mut nesting_level = 0;
    let mut indent_level = 0;
    while !reached_root {
        // TODO: newlines
        if cursor.node().kind() == "(" {
            indent_level += 1;
        }
        if cursor.node().kind() == ")" {
            indent_level -= 1;
        }
        if cursor.node().kind() == "[" {
            indent_level += 1;
        }
        if cursor.node().kind() == "]" {
            indent_level -= 1;
        }
        if nesting_level == 1 {
            output.push_str("\n\n")
        }
        if cursor.node().kind() == "field_definition" {
            output.push('\n');
            output.push_str(&"  ".repeat(indent_level));
        }
        if cursor.node().kind() == "predicate" {
            output.push('\n');
            output.push_str(&"  ".repeat(indent_level));
        }
        if cursor.node().kind() == "capture" && !check_parent("parameters", cursor.node()) {
            output.push(' ');
        }

        if cursor.node().kind() == "anonymous_node" && check_parent("list", cursor.node()) {
            output.push('\n');
            output.push_str(&"  ".repeat(indent_level));
        }

        if cursor.node().kind() == "]" && check_parent("list", cursor.node()) {
            output.push('\n');
            output.push_str(&"  ".repeat(indent_level));
        }

        if cursor.node().kind() == "identifier"
            && check_parent("anonymous_node", cursor.node())
            && !check_parent("list", cursor.node().parent().unwrap())
        {
            output.push('\n');
            output.push_str(&"  ".repeat(indent_level));
        }

        if check_parent("parameters", cursor.node()) {
            output.push(' ')
        }

        if check_parent("named_node", cursor.node()) && cursor.node().kind() == "named_node" {
            output.push('\n');
            output.push_str(&"  ".repeat(indent_level));
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
