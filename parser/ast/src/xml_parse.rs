#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum XmlWhitespaceOption {
    #[default]
    Strip,
    Preserve,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlParse {
    kind: XmlNodeKind,
    content: ExprNode,
    whitespace: XmlWhitespaceOption
}

impl XmlParse {
    pub fn new(kind: XmlNodeKind, content: ExprNode, whitespace: XmlWhitespaceOption) -> Self {
        Self { kind, content, whitespace }
    }

    pub fn kind(&self) -> XmlNodeKind {
        self.kind
    }

    pub fn content(&self) -> &ExprNode {
        &self.content
    }

    pub fn whitespace(&self) -> XmlWhitespaceOption {
        self.whitespace
    }
}

use crate::ExprNode;
use crate::XmlNodeKind;
