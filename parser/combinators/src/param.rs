pub fn param(ctx: &mut ParserContext) -> scan::Result<i32> {
    ctx.stream_mut().consume(|tok| match tok {
        Param { index } => Some(*index),
        _ => None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_param() {
        test_parser!(
            source = "$3",
            parser = param,
            expected = 3
        )
    }
}

use crate::ParserContext;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenValue::Param;
