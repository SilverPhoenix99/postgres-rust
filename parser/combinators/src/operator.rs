impl Combinator for OperatorKind {
    type Output = OperatorKind;

    fn parse(&self, ctx: &mut ParserContext) -> scan::Result<Self::Output> {
        ctx.stream_mut().consume(|tok| match tok {
            Operator(op) if *op == *self => Some(*op),
            _ => None,
        })
    }
}

use crate::Combinator;
use pg_lexer::OperatorKind;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue::Operator;
use pg_parser_core::ParserContext;
