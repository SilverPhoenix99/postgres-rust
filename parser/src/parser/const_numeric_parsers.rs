impl Parser<'_> {

    /// Alias: `NumericOnly`
    pub(in crate::parser) fn signed_number(&mut self) -> ScanResult<SignedNumber> {
        use RawTokenKind::{Minus, Plus};
        const FN_NAME: &str = "postgres_parser::parser::Parser::signed_number";

        // ('+' | '-')? (ICONST | FCONST)

        let sign = self.buffer
            .consume(|tok| matches!(tok, Minus | Plus))
            .no_match_to_option()?;

        let number = self.unsigned_number();

        let number = if sign.is_some() {
            number.required(fn_info!(FN_NAME))?
        }
        else {
            number?
        };

        let negative = sign.is_some_and(|s| s == Minus);

        let value = match number {
            UnsignedNumber::IntegerConst(int) => {
                let mut int: i32 = int.into();
                if negative {
                    int = -int;
                }
                SignedNumber::IntegerConst(int)
            },
            UnsignedNumber::NumericConst { value, radix } => {
                SignedNumber::NumericConst { value, radix, negative }
            }
        };

        Ok(value)
    }

    pub(in crate::parser) fn unsigned_number(&mut self) -> ScanResult<UnsignedNumber> {

        // ICONST | FCONST

        self.buffer.consume_with_slice(|(tok, slice, _)| {
            let NumberLiteral { radix } = tok else { return None };
            parse_number(slice, radix)
        })
    }

    /// Alias: `ICONST`
    pub(in crate::parser) fn i32_literal(&mut self) -> ScanResult<i32> {

        self.buffer.consume_with_slice(|(tok, slice, _)| {
            let NumberLiteral { radix } = tok else { return None };
            let Some(UnsignedNumber::IntegerConst(int)) = parse_number(slice, radix) else { return None };
            Some(int.into())
        })
    }

    /// Alias: `SignedIconst`
    pub(in crate::parser) fn signed_i32_literal(&mut self) -> ScanResult<i32> {
        use RawTokenKind::{Minus, Plus};
        const FN_NAME: &str = "postgres_parser::parser::Parser::signed_i32_literal";

        // ('+' | '-')? ICONST

        let sign = self.buffer
            .consume(|tok| matches!(tok, Minus | Plus))
            .no_match_to_option()?;

        let num = self.i32_literal();

        let Some(sign) = sign else { return num };

        // If sign is Some(_), then ICONST is required
        let mut num = num.required(fn_info!(FN_NAME))?;

        if sign == Minus {
            num = -num;
        }

        Ok(num)
    }
}

fn parse_number(value: &str, radix: u32) -> Option<UnsignedNumber> {
    use UnsignedNumber::*;

    let value = value.replace("_", "");

    if let Ok(int) = i32::from_str_radix(&value, radix) {
        // SAFETY: `0 <= int <= i32::MAX`
        Some(IntegerConst(int.into()))
    }
    else {
        Some(NumericConst { radix, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case( "1.01", SignedNumber::NumericConst { value: "1.01".into(), radix: 10, negative: false })]
    #[test_case("+2.02", SignedNumber::NumericConst { value: "2.02".into(), radix: 10, negative: false })]
    #[test_case("-3.03", SignedNumber::NumericConst { value: "3.03".into(), radix: 10, negative: true })]
    #[test_case(  "101", SignedNumber::IntegerConst(101))]
    #[test_case( "+202", SignedNumber::IntegerConst(202))]
    #[test_case( "-303", SignedNumber::IntegerConst(-303))]
    fn test_signed_number(source: &str, expected: SignedNumber) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.signed_number();
        assert_eq!(Ok(expected), actual);
    }

    #[test_case("1.1", UnsignedNumber::NumericConst { value: "1.1".into(), radix: 10 })]
    #[test_case("11",  UnsignedNumber::IntegerConst(11.into()))]
    fn test_unsigned_number(source: &str, expected: UnsignedNumber) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.unsigned_number();
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_i32_literal() {
        let mut parser = Parser::new("123", DEFAULT_CONFIG);
        assert_eq!(Ok(123), parser.i32_literal());
    }

    #[test_case("-123", -123)]
    #[test_case("+321", 321)]
    fn test_signed_i32_literal(source: &str, expected: i32) {
        let mut parser = Parser::new(source, DEFAULT_CONFIG);
        let actual = parser.signed_i32_literal();
        assert_eq!(Ok(expected), actual);
    }
}

use crate::{
    lexer::RawTokenKind::{self, NumberLiteral},
    parser::{
        ast_node::{SignedNumber, UnsignedNumber},
        result::{Required, ScanResult, ScanResultTrait},
        token_buffer::{SlicedTokenConsumer, TokenConsumer},
        Parser
    }
};
use postgres_basics::fn_info;
