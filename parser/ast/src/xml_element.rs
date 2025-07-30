#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlElement {
    name: Str,
    attributes: Option<Vec<ExprNode>>,
    args: Option<Vec<ExprNode>>,
}

use crate::ExprNode;
use pg_basics::Str;
