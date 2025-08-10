pub fn user_defined_operator(ctx: &mut ParserContext) -> scan::Result<Box<str>> {
    ctx.stream_mut().consume(|tok| {
        let UserDefinedOperator(value) = tok else { return None };
        Some(mem::take(value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_user_defined_op() {
        test_parser!(
            source = "~@",
            parser = user_defined_operator,
            expected = "~@"
        );
    }
}

use crate::ParserContext;
use core::mem;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue::UserDefinedOperator;
