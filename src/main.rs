use tree_sitter::{Language, Parser};

fn main() {
    let mut parser = Parser::new();
    extern "C" {
        fn tree_sitter_query() -> Language;
    }

    let language = unsafe { tree_sitter_query() };
    parser.set_language(language).unwrap();
    let source_code = "(block)@test
(mod_item
 name: (identifier)@namespace)
";

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    let mut output = String::new();
    for child in root_node.children(&mut tree.walk()) {
        println!("kind:{}", child.kind());
        println!("text:{}", child.utf8_text(source_code.as_bytes()).unwrap());
        if child.kind() == "field_definition" {
            println!("got it");
            output.push_str("\n  ")
        }
        if child.child_count() == 0 {
            output.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
        }
        for child_child in child.children(&mut tree.walk()) {
            println!("  kind:{}", child_child.kind());
            println!(
                "  text:{}",
                child_child.utf8_text(source_code.as_bytes()).unwrap()
            );
            println!("{}", child.kind() == "field_definition");
            println!("==========");
            if child.kind() == "field_definition" {
                println!("got it");
                output.push_str("\n  ")
            }
            if child_child.child_count() == 0 {
                output.push_str(child_child.utf8_text(source_code.as_bytes()).unwrap());
            }
            for child_child_child in child_child.children(&mut tree.walk()) {
                println!("    kind:{}", child_child_child.kind());
                println!(
                    "    text:{}",
                    child_child_child.utf8_text(source_code.as_bytes()).unwrap()
                );
                if child.kind() == "field_definition" {
                    println!("got it");
                    output.push_str("\n  ")
                }
                if child_child_child.child_count() == 0 {
                    output.push_str(child_child_child.utf8_text(source_code.as_bytes()).unwrap());
                }
                for child_child_child_child in child_child_child.children(&mut tree.walk()) {
                    println!("      kind:{}", child_child_child_child.kind());
                    println!(
                        "      text:{}",
                        child_child_child_child
                            .utf8_text(source_code.as_bytes())
                            .unwrap()
                    );
                    if child.kind() == "field_definition" {
                        println!("got it");
                        output.push_str("\n  ")
                    }
                    if child_child_child_child.child_count() == 0 {
                        output.push_str(
                            child_child_child_child
                                .utf8_text(source_code.as_bytes())
                                .unwrap(),
                        );
                    }
                    for child_child_child_child_child in
                        child_child_child_child.children(&mut tree.walk())
                    {
                        println!("        kind:{}", child_child_child_child_child.kind());
                        println!(
                            "        text:{}",
                            child_child_child_child_child
                                .utf8_text(source_code.as_bytes())
                                .unwrap()
                        );
                        if child.kind() == "field_definition" {
                            println!("got it");
                            output.push_str("\n  ")
                        }
                        if child_child_child_child_child.child_count() == 0 {
                            output.push_str(
                                child_child_child_child_child
                                    .utf8_text(source_code.as_bytes())
                                    .unwrap(),
                            );
                        }
                    }
                }
            }
        }
        output.push('\n');
        output.push('\n');
    }
    println!("{output}");
}
