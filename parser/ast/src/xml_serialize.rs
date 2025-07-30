#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlSerialize {
    kind: XmlNodeKind,
    value: ExprNode,
    type_name: TypeName,
    indent: bool,
}

use crate::XmlNodeKind;
use crate::ExprNode;
use crate::TypeName;
