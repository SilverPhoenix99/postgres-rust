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

use postgres_parser_ast::UnsignedNumber;
use postgres_parser_lexer::BitStringKind;
use postgres_parser_lexer::Keyword;
use postgres_parser_lexer::OperatorKind;
