pub mod numeric_spec;

pub use self::numeric_spec::NumericSpec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Cow<'static, str>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SystemType {
    // Character types:
    // If limited, the maximum is 10MB == 10,485,760
    // see https://www.postgresql.org/docs/current/datatype-character.html
    Varchar(Option<u32>),
    Bpchar(Option<u32>),
    // Numeric types:
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Numeric(Vec<AstNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstLiteral {
    StringLiteral(String),
    BitStringLiteral(BitBox),
    IntegerLiteral(i32),
    FloatLiteral(f64),
    NumericLiteral(String), // TODO: Replace with some kind of BigDecimal
    BooleanLiteral(bool),
    NullLiteral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Literal(AstLiteral),
    SystemType(SystemType),
    TypeCast(/* ??? */)
}

use bitvec::boxed::BitBox;
use std::borrow::Cow;
