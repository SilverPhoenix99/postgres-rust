/// Alias: `NumericOnly`
pub(super) fn signed_number(ctx: &mut ParserContext) -> scan::Result<SignedNumber> {

    // ('+' | '-')? (ICONST | FCONST)

    let (sign, num) = seq!(sign.optional(), number)
        .parse(ctx)?;

    let mut num = SignedNumber::from(num);

    if let Some(Minus) = sign {
        num = -num;
    }

    Ok(num)
}

/// Alias: `ICONST`
pub(super) fn i32_literal(ctx: &mut ParserContext) -> scan::Result<i32> {
    integer(ctx).map(i32::from)
}

/// Alias: `SignedIconst`
pub(super) fn signed_i32_literal(ctx: &mut ParserContext) -> scan::Result<i32> {

    // ('+' | '-')? ICONST

    let (sign, mut int) = seq!(sign.optional(), i32_literal).parse(ctx)?;

    if let Some(Minus) = sign {
        int = -int;
    }

    Ok(int)
}

/// '+' | '-'
fn sign(ctx: &mut ParserContext) -> scan::Result<OperatorKind> {

    alt!(Minus, Plus)
        .parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::Number;
    use pg_basics::NumberRadix::Decimal;
    use test_case::test_matrix;

    #[test_matrix( "1.01" => Ok(SignedNumber::NumericConst(Number::new("1.01".into(), Decimal))))]
    #[test_matrix("+2.02" => Ok(SignedNumber::NumericConst(Number::new("2.02".into(), Decimal))))]
    #[test_matrix("-3.03" => Ok(SignedNumber::NumericConst(-Number::new("3.03".into(), Decimal))))]
    #[test_matrix(  "101" => Ok(SignedNumber::IntegerConst(101)))]
    #[test_matrix( "+202" => Ok(SignedNumber::IntegerConst(202)))]
    #[test_matrix( "-303" => Ok(SignedNumber::IntegerConst(-303)))]
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

    #[test_matrix("-123" => Ok(-123))]
    #[test_matrix("+321" => Ok(321))]
    fn test_signed_i32_literal(source: &str) -> scan::Result<i32> {
        test_parser!(source, signed_i32_literal)
    }
}

use crate::alt;
use crate::combinators::core::integer;
use crate::combinators::core::number;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::SignedNumber;
use pg_lexer::OperatorKind;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::Plus;
use pg_parser_core::scan;
