use tree_sitter::Node;

pub struct NodeItem<'a> {
    node: Node<'a>,
    nesting_level: u8,
}

impl<'a> NodeItem<'a> {
    pub fn new(node: Node<'a>, nesting_level: u8) -> Self {
        Self {
            node,
            nesting_level,
        }
    }

    fn parent_type(&self) -> Option<&str> {
        self.node.parent().map(|parent_node| parent_node.kind())
    }

    pub fn parent_equals(&self, parent_type: &str) -> bool {
        self.parent_type()
            .map_or(false, |par_type| par_type == parent_type)
    }

    pub fn kind(&self) -> &str {
        self.node.kind()
    }

    pub fn nesting_level(&self) -> u8 {
        self.nesting_level
    }

    pub fn node(&self) -> Node {
        self.node
    }
}
