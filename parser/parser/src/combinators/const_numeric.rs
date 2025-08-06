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
    #[allow(unused_imports)]
    use pg_basics::NumberRadix::Decimal;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case( "1.01" => Ok(SignedNumber::NumericConst { value: "1.01".into(), radix: Decimal, negative: false }))]
    #[test_case("+2.02" => Ok(SignedNumber::NumericConst { value: "2.02".into(), radix: Decimal, negative: false }))]
    #[test_case("-3.03" => Ok(SignedNumber::NumericConst { value: "3.03".into(), radix: Decimal, negative: true }))]
    #[test_case(  "101" => Ok(SignedNumber::IntegerConst(101)))]
    #[test_case( "+202" => Ok(SignedNumber::IntegerConst(202)))]
    #[test_case( "-303" => Ok(SignedNumber::IntegerConst(-303)))]
    fn test_signed_number(source: &str) -> scan::Result<SignedNumber> {
        test_parser!(source, signed_number)
    }

    #[test]
    fn test_i32_literal() {
        test_parser!(
            source = "123",
            parser = i32_literal,
            expected = 123
        )
    }

    #[test_case("-123" => Ok(-123))]
    #[test_case("+321" => Ok(321))]
    fn test_signed_i32_literal(source: &str) -> scan::Result<i32> {
        test_parser!(source, signed_i32_literal)
    }
}

use crate::combinators::foundation::integer;
use crate::combinators::foundation::number;
use crate::combinators::foundation::seq;
use crate::combinators::sign;
use pg_ast::SignedNumber;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Minus;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
