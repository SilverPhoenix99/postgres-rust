/// Alias: `NumericOnly`
pub(super) fn signed_number(stream: &mut TokenStream) -> scan::Result<SignedNumber> {

    // ('+' | '-')? (ICONST | FCONST)

    let (sign, num) = seq!(sign.optional(), number)
        .parse(stream)?;

    let mut num = SignedNumber::from(num);

    if let Some(Minus) = sign {
        num = -num;
    }

    Ok(num)
}

/// Alias: `ICONST`
pub(super) fn i32_literal(stream: &mut TokenStream) -> scan::Result<i32> {
    integer(stream).map(i32::from)
}

/// Alias: `SignedIconst`
pub(super) fn signed_i32_literal(stream: &mut TokenStream) -> scan::Result<i32> {

    // ('+' | '-')? ICONST

    let (sign, mut int) = seq!(sign.optional(), i32_literal).parse(stream)?;

    if let Some(Minus) = sign {
        int = -int;
    }

    Ok(int)
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
        let actual = signed_number(&mut stream);
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_i32_literal() {
        let mut stream = TokenStream::new("123", DEFAULT_CONFIG);
        assert_eq!(Ok(123), i32_literal(&mut stream));
    }

    #[test_case("-123", -123)]
    #[test_case("+321", 321)]
    fn test_signed_i32_literal(source: &str, expected: i32) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = signed_i32_literal(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::integer;
use crate::combinators::foundation::number;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::sign;
use crate::stream::TokenStream;
use pg_ast::SignedNumber;
use pg_lexer::OperatorKind::Minus;
use pg_parser_core::scan;
