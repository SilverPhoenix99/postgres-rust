impl Combinator for OperatorKind {
    type Output = OperatorKind;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        stream.consume(|tok| match tok {
            Operator(op) if *op == *self => Some(*op),
            _ => None,
        })
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Operator;
use pg_lexer::OperatorKind;
