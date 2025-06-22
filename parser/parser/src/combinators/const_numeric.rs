/// Alias: `NumericOnly`
pub(super) fn signed_number() -> impl Combinator<Output = SignedNumber> {

    // ('+' | '-')? (ICONST | FCONST)

    parser(|stream| {

        let sign = sign().maybe_match().parse(stream)?;
        let num = number().map(SignedNumber::from);

        let negative = match sign {
            None => return num.parse(stream),
            Some(sign) => sign == Minus,
        };

        let mut num = num.required().parse(stream)?;
        if negative {
            num = num.neg();
        }

        Ok(num)
    })
}

/// Alias: `ICONST`
pub(super) fn i32_literal() -> impl Combinator<Output = i32> {
    integer.map(i32::from)
}

/// Alias: `SignedIconst`
pub(super) fn signed_i32_literal() -> impl Combinator<Output = i32> {

    // ('+' | '-')? ICONST

    parser(|stream| {

        let sign = sign().maybe_match().parse(stream)?;

        let int = match sign {
            None => i32_literal().parse(stream)?,
            Some(sign) => {
                let mut num = i32_literal()
                    .required()
                    .parse(stream)?;
                if sign == Minus {
                    num = -num;
                }
                num
            }
        };

        Ok(int)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    #[allow(unused_imports)]
    use pg_basics::NumberRadix::Decimal;
    use test_case::test_case;

    #[test_case( "1.01", SignedNumber::NumericConst { value: "1.01".into(), radix: Decimal, negative: false })]
    #[test_case("+2.02", SignedNumber::NumericConst { value: "2.02".into(), radix: Decimal, negative: false })]
    #[test_case("-3.03", SignedNumber::NumericConst { value: "3.03".into(), radix: Decimal, negative: true })]
    #[test_case(  "101", SignedNumber::IntegerConst(101))]
    #[test_case( "+202", SignedNumber::IntegerConst(202))]
    #[test_case( "-303", SignedNumber::IntegerConst(-303))]
    fn test_signed_number(source: &str, expected: SignedNumber) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = signed_number().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_i32_literal() {
        let mut stream = TokenStream::new("123", DEFAULT_CONFIG);
        assert_eq!(Ok(123), i32_literal().parse(&mut stream));
    }

    #[test_case("-123", -123)]
    #[test_case("+321", 321)]
    fn test_signed_i32_literal(source: &str, expected: i32) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = signed_i32_literal().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::integer;
use crate::combinators::foundation::number;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::sign;
use core::ops::Neg;
use pg_ast::SignedNumber;
use pg_lexer::OperatorKind::Minus;
