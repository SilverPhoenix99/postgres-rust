/// Returns `ICONST | FCONST`.
pub(in crate::parser) fn number() -> NumberCombi {
    NumberCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct NumberCombi;

impl Combinator for NumberCombi {
    type Output = UnsignedNumber;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume_with_slice(|(tok, slice, _)| {
            let NumberLiteral(radix) = tok else { return None };
            parse_number(slice, radix)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("1.1", UnsignedNumber::NumericConst { value: "1.1".into(), radix: crate::NumberRadix::Decimal })]
    #[test_case("11",  UnsignedNumber::IntegerConst(11.into()))]
    fn test_unsigned_number(source: &str, expected: UnsignedNumber) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = number().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::lexer::RawTokenKind::NumberLiteral;
use crate::parser::ast_node::UnsignedNumber;
use crate::parser::combinators::{integer::parse_number, Combinator};
use crate::parser::result::ScanResult;
use crate::parser::token_stream::{SlicedTokenConsumer, TokenStream};
