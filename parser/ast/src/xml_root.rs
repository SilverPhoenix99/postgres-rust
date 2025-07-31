#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum XmlStandalone {
    Yes,
    No,
    NoValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlRoot {
    content: ExprNode,
    version: ExprNode,
    standalone: Option<XmlStandalone>,
}

impl XmlRoot {
    pub fn new(content: ExprNode, version: ExprNode) -> Self {
        Self {
            content,
            version,
            standalone: None,
        }
    }

    pub fn content(&self) -> &ExprNode {
        &self.content
    }

    pub fn version(&self) -> &ExprNode {
        &self.version
    }

    pub fn set_standalone(&mut self, standalone: Option<XmlStandalone>) -> &mut Self {
        self.standalone = standalone;
        self
    }

    pub fn with_standalone(mut self, standalone: XmlStandalone) -> Self {
        self.standalone = Some(standalone);
        self
    }

    pub fn standalone(&self) -> Option<XmlStandalone> {
        self.standalone
    }
}

use crate::ExprNode;
