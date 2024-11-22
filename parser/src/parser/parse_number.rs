pub(in crate::parser) fn parse_number(value: &str, radix: NumberRadix) -> UnsignedNumber {

    let value = value.replace("_", "");

    if let Ok(int) = i32::from_str_radix(&value, radix as u32) {
        // SAFETY: `0 <= int <= i32::MAX`
        IntegerConst(int.into())
    }
    else {
        NumericConst {
            radix,
            value: value.into_boxed_str()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test] #[ignore]
    fn test_parse_number() {
        todo!()
    }
}

use crate::parser::ast_node::UnsignedNumber;
use crate::parser::ast_node::UnsignedNumber::{IntegerConst, NumericConst};
use crate::NumberRadix;
