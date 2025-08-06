pub(in crate::combinators) fn param(stream: &mut TokenStream) -> scan::Result<i32> {
    stream.consume(|tok| match tok {
        Param { index } => Some(*index),
        _ => None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_param() {
        test_parser!(
            source = "$3",
            parser = param,
            expected = 3
        )
    }
}

use pg_parser_core::scan;
use pg_parser_core::stream::TokenConsumer;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Param;
