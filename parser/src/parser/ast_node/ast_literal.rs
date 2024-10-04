#[derive(Debug, Clone, PartialEq)]
pub enum AstLiteral {
    StringLiteral(String),
    BitStringLiteral(BitBox),
    IntegerLiteral(i32),
    FloatLiteral(f64),
    NumericLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
}

use bitvec::boxed::BitBox;
