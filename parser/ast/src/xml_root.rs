#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum XmlStandalone {
    Yes,
    No,
    NoValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlRoot {
    content: ExprNode,
    version: Option<ExprNode>,
    standalone: Option<XmlStandalone>,
}

use crate::ExprNode;
