pub(in crate::combinators) fn param(stream: &mut TokenStream) -> scan::Result<i32> {
    stream.consume(|tok| match tok {
        Param { index } => Some(*index),
        _ => None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_param() {
        test_parser!(
            source = "$3",
            parser = param,
            expected = 3
        )
    }
}

use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Param;
use pg_parser_core::scan;
