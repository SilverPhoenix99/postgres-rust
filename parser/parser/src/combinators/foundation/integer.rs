/// Alias: `ICONST`
pub(in crate::combinators) fn integer() -> IntegerCombi {
    IntegerCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct IntegerCombi;

impl Combinator for IntegerCombi {
    type Output = NonNegative;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            let UnsignedNumber(IntegerConst(value)) = tok else { return None };
            Some(*value)
        })
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::ScanResult;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::UnsignedNumber;
use postgres_basics::NonNegative;
use postgres_parser_ast::UnsignedNumber::IntegerConst;
