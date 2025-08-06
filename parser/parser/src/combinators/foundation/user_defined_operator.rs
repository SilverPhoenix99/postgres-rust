pub(in crate::combinators) fn user_defined_operator(stream: &mut TokenStream<'_>) -> scan::Result<Box<str>> {
    stream.consume(|tok| {
        let UserDefinedOperator(value) = tok else { return None };
        Some(mem::take(value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_user_defined_op() {
        test_parser!(
            source = "~@",
            parser = user_defined_operator,
            expected = "~@"
        );
    }
}

use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::UserDefinedOperator;
use core::mem;
use pg_parser_core::scan;
