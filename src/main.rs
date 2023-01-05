use tree_sitter::{Language, Parser};

fn main() {
    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_query() -> Language;
    }

    let language = unsafe { tree_sitter_query() };
    parser.set_language(language).unwrap();
    let source_code = "(block)@test";

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    for child in root_node.children(&mut tree.walk()) {
        println!("{}", child.kind());
        println!("text:{}", child.utf8_text(source_code.as_bytes()).unwrap());
        for child_child in child.children(&mut tree.walk()) {
            println!("  {}", child_child.kind());
            println!(
                "  text:{}",
                child_child.utf8_text(source_code.as_bytes()).unwrap()
            );
            for child_child_child in child_child.children(&mut tree.walk()) {
                println!("    {}", child_child_child.kind());
                println!(
                    "    text:{}",
                    child_child_child.utf8_text(source_code.as_bytes()).unwrap()
                );
            }
        }
    }
}
