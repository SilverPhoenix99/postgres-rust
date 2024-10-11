/// Redundant enum, to avoid using `unreachable!()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) enum CharacterSystemType {
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
    Bit(Vec<ExprNode>),
    Varbit(Vec<ExprNode>),
    // Numeric types:
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Numeric(Vec<ExprNode>),
}

use crate::parser::ExprNode;
