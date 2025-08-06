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
use pg_lexer::OperatorKind;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Operator;
