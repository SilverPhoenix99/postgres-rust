#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum TokenValue {
    Param { index: i32 },
    Keyword(Keyword),
    Operator(OperatorKind),
    UserDefinedOperator(Box<str>),
    UnsignedNumber(UnsignedNumber),
    Identifier(Box<str>),
    String(Box<str>),
    BitString { value: Box<str>, kind: BitStringKind },
}

use crate::lexer::BitStringKind;
use crate::lexer::Keyword;
use crate::lexer::OperatorKind;
use crate::parser::ast_node::UnsignedNumber;
