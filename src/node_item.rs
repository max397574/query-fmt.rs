use tree_sitter::Node;

pub struct NestedNode<'a> {
    pub inner: Node<'a>,
    pub nesting_level: u8,
    pub first_node: bool,
}

impl<'a> NestedNode<'a> {
    #[must_use]
    pub const fn new(node: Node<'a>, nesting_level: u8, first_node: bool) -> Self {
        Self {
            inner: node,
            nesting_level,
            first_node,
        }
    }

    pub fn parent_type(&self) -> Option<&'static str> {
        self.inner.parent_type()
    }

    pub fn parent_eq(&self, parent_type: &str) -> bool {
        self.inner.parent_eq(parent_type)
    }

    pub fn grandparent_type(&self) -> Option<&'static str> {
        self.inner.grandparent_type()
    }

    pub fn grandparent_eq(&self, grandparent_type: &str) -> bool {
        self.inner.grandparent_eq(grandparent_type)
    }

    pub fn ancestor_eq(&self, ancestor_type: &str) -> bool {
        self.inner.ancestor_eq(ancestor_type)
    }

    pub fn kind(&self) -> &str {
        self.inner.kind()
    }
}

#[extend::ext(name = NodeExt)]
pub impl<'a> Node<'a> {
    fn parent_type(&self) -> Option<&'static str> {
        self.parent().map(|parent_node| parent_node.kind())
    }

    fn grandparent_type(&self) -> Option<&'static str> {
        self.parent()
            .and_then(|parent_node| parent_node.parent())
            .map(|grandparent_node| grandparent_node.kind())
    }

    fn parent_eq(&self, parent_type: &str) -> bool {
        self.parent_type()
            .map_or(false, |par_type| par_type == parent_type)
    }

    fn grandparent_eq(&self, grandparent_type: &str) -> bool {
        self.grandparent_type()
            .map_or(false, |par_type| par_type == grandparent_type)
    }

    fn ancestor_eq(&self, ancestor_type: &str) -> bool {
        let mut parent = self.parent();
        while let Some(parent_node) = parent {
            if parent_node.kind() == ancestor_type {
                return true;
            }
            parent = parent_node.parent();
        }
        false
    }
}
