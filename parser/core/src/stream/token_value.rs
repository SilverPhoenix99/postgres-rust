#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenValue {
    Param { index: i32 },
    Keyword(Keyword),
    Operator(OperatorKind),
    UserDefinedOperator(Box<str>),
    UnsignedNumber(UnsignedNumber),
    Identifier(Box<str>),
    String(Box<str>),
    BitString { value: Box<str>, kind: BitStringKind },
}

use pg_basics::UnsignedNumber;
use pg_lexer::BitStringKind;
use pg_lexer::Keyword;
use pg_lexer::OperatorKind;
