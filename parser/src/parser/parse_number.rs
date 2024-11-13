pub(in crate::parser) fn parse_number(value: &str, radix: u32) -> UnsignedNumber {
    use crate::parser::ast_node::UnsignedNumber::*;

    let value = value.replace("_", "");

    if let Ok(int) = i32::from_str_radix(&value, radix) {
        // SAFETY: `0 <= int <= i32::MAX`
        IntegerConst(int.into())
    }
    else {
        NumericConst { radix, value }
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
