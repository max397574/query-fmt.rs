use crate::node_item::NodeItem;
use tree_sitter::TreeCursor;

pub struct QueryTree<'a> {
    pub cursor: TreeCursor<'a>,
    pub reached_root: bool,
    pub nesting_level: u8,
}
impl<'a> Iterator for &mut QueryTree<'a> {
    type Item = NodeItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_root {
            return None;
        }
        if self.cursor.goto_first_child() {
            self.nesting_level += 1;
            return Some(NodeItem::new(self.cursor.node(), self.nesting_level));
        }
        if self.cursor.goto_next_sibling() {
            return Some(NodeItem::new(self.cursor.node(), self.nesting_level));
        }
        let mut retracing = true;
        while retracing {
            if !self.cursor.goto_parent() {
                retracing = false;
                self.reached_root = true;
            } else {
                self.nesting_level -= 1;
            }
            if self.cursor.goto_next_sibling() {
                retracing = false;
            }
        }
        Some(NodeItem::new(self.cursor.node(), self.nesting_level))
    }
}
