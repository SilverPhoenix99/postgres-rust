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

/// Redundant enum, to avoid using `unreachable!()`.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum CharacterSystemType {
    Varchar,
    Bpchar
}

#[derive(Debug, Clone, PartialEq)]
pub enum SystemType {
    // Character types:
    // If limited, the maximum is 10MB == 10,485,760
    // see https://www.postgresql.org/docs/current/datatype-character.html
    Varchar(Option<i32>),
    /// Blank-Padded Character string
    Bpchar(Option<i32>),
    // Bit string types:
    Bit(Vec<AstNode>),
    Varbit(Vec<AstNode>),
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
