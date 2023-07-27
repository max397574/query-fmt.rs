use crate::node_item::NestedNode;
use tree_sitter::TreeCursor;

pub struct QueryTree<'a> {
    pub cursor: TreeCursor<'a>,
    pub reached_root: bool,
    pub nesting_level: u8,
    pub first_node: bool,
}

impl<'a> Iterator for &mut QueryTree<'a> {
    type Item = NestedNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_root {
            return None;
        }

        if self.cursor.goto_first_child() {
            self.nesting_level += 1;
            return Some(NestedNode::new(
                self.cursor.node(),
                self.nesting_level,
                self.first_node,
            ));
        }

        if self.cursor.goto_next_sibling() {
            return Some(NestedNode::new(
                self.cursor.node(),
                self.nesting_level,
                false,
            ));
        }

        let mut retracing = true;
        while retracing {
            if self.cursor.goto_parent() {
                self.first_node = false;
                self.nesting_level -= 1;
            } else {
                retracing = false;
                self.reached_root = true;
            }
            if self.cursor.goto_next_sibling() {
                self.first_node = false;
                retracing = false;
            }
        }

        Some(NestedNode::new(
            self.cursor.node(),
            self.nesting_level,
            self.first_node,
        ))
    }
}
