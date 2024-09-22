pub mod numeric_spec;

pub use self::numeric_spec::NumericSpec;
use bitvec::boxed::BitBox;
use std::borrow::Cow;
use std::num::NonZero;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoleSpec {
    Public,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Name(Cow<'static, str>),
}

// If limited, the maximum is 10MB == 10,485,760
// see https://www.postgresql.org/docs/current/datatype-character.html
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CharacterType {
    Varchar(Option<NonZero<u32>>),
    Bpchar(Option<NonZero<u32>>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NumericType {
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Numeric(NumericSpec),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SystemType {
    Character(CharacterType),
    Numeric(NumericType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    StringLiteral(String),
    BitStringLiteral(BitBox),
    IntegerLiteral(i32),
    FloatLiteral(f64),
    NumericLiteral(String), // TODO: Replace with some kind of BigDecimal
    BooleanLiteral(bool),
    NullLiteral,
}
