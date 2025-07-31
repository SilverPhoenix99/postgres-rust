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

use crate::ExprNode;
use crate::XmlNodeKind;
