
#[derive(Debug, Clone, PartialEq)]
pub enum ConstExpr {
    TypecastConst(TypeName, String),
    StringConst(String),
    BinaryStringConst(String),
    HexStringConst(String),
    IntegerConst(i32),
    NumericConst { radix: u32, value: String },
    BooleanConst(bool),
    NullConst,
}

impl From<UnsignedNumber> for ConstExpr {
    fn from(value: UnsignedNumber) -> Self {
        use UnsignedNumber::*;
        match value {
            // SAFETY: `int` is originally parsed by `i32::from_str_radix()`, so `0 <= int <= i32::MAX`
            IntegerConst(int) => Self::IntegerConst(int as i32),
            NumericConst { value, radix } => Self::NumericConst { radix, value }
        }
    }
}

use crate::parser::ast_node::{TypeName, UnsignedNumber};
