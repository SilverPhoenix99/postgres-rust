/// Alias: `ICONST`
pub(in crate::combinators) fn integer() -> IntegerCombi {
    IntegerCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct IntegerCombi;

impl Combinator for IntegerCombi {
    type Output = NonNegative;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        stream.consume(|tok| {
            let UnsignedNumber(IntegerConst(value)) = tok else { return None };
            Some(*value)
        })
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::UnsignedNumber;
use pg_ast::UnsignedNumber::IntegerConst;
use pg_basics::NonNegative;
