pub(in crate::combinators) fn param(stream: &mut TokenStream) -> scan::Result<i32> {
    stream.consume(|tok| match tok {
        Param { index } => Some(*index),
        _ => None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_param() {
        let mut stream = TokenStream::new("$3", DEFAULT_CONFIG);
        assert_eq!(Ok(3), param(&mut stream))
    }
}

use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Param;
use pg_parser_core::scan;
