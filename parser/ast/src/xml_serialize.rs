#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlSerialize {
    kind: XmlNodeKind,
    content: ExprNode,
    type_name: TypeName,
    indent: bool,
}

impl XmlSerialize {
    pub fn new(kind: XmlNodeKind, content: ExprNode, type_name: TypeName) -> Self {
        Self {
            kind,
            content,
            type_name,
            indent: false,
        }
    }

    pub fn kind(&self) -> XmlNodeKind {
        self.kind
    }

    pub fn content(&self) -> &ExprNode {
        &self.content
    }

    pub fn type_name(&self) -> &TypeName {
        &self.type_name
    }

    pub fn set_indent(&mut self, indent: bool) -> &mut Self {
        self.indent = indent;
        self
    }

    pub fn with_indent(mut self, indent: bool) -> Self {
        self.indent = indent;
        self
    }

    pub fn indent(&self) -> bool {
        self.indent
    }
}

use crate::XmlNodeKind;
use crate::ExprNode;
use crate::TypeName;
