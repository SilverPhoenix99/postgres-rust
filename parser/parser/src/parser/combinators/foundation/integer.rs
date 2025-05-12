/// Alias: `ICONST`
pub(in crate::parser::combinators) fn integer() -> IntegerCombi {
    IntegerCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser::combinators) struct IntegerCombi;

impl Combinator for IntegerCombi {
    type Output = NonNegative;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            let UnsignedNumber(IntegerConst(value)) = tok else { return None };
            Some(*value)
        })
    }
}

use crate::parser::ast_node::UnsignedNumber::IntegerConst;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::token_value::TokenValue::UnsignedNumber;
use postgres_basics::NonNegative;
