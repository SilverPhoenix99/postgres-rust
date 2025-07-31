#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlExists {
    path_spec: ExprNode,
    content: ExprNode
}

impl XmlExists {
    pub fn new(path_spec: ExprNode, content: ExprNode) -> Self {
        Self { path_spec, content }
    }

    pub fn path_spec(&self) -> &ExprNode {
        &self.path_spec
    }

    pub fn content(&self) -> &ExprNode {
        &self.content
    }
}

use crate::ExprNode;
