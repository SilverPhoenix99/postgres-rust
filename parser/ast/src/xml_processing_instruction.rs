#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlProcessingInstruction {
    name: Str,
    value: Option<ExprNode>,
}

use crate::ExprNode;
use pg_basics::Str;
