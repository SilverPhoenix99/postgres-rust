#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum XmlNodeKind {
    Document,
    Content,
}

impl XmlNodeKind {
    #[inline(always)]
    pub fn is_document(&self) -> bool {
        *self == Self::Document
    }

    #[inline(always)]
    pub fn is_content(&self) -> bool {
        *self == Self::Content
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum XmlStandalone {
    Yes,
    No,
    NoValue,
    Omitted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlElement {
    name: Str,
    attributes: Vec<ExprNode>,
    args: Vec<ExprNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlParse {
    text: ExprNode,
    kind: XmlNodeKind,
    preserve_whitespace: bool
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlProcessingInstruction {
    name: Str,
    args: Option<ExprNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlRoot {
    version: Option<ExprNode>,
    standalone: XmlStandalone,
    xml: ExprNode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlSerialize {
    kind: XmlNodeKind,
    x: ExprNode,
    type_name: SystemType,
    indent: bool,
}

use crate::parser::ast_node::{ExprNode, SystemType};
use postgres_basics::Str;
