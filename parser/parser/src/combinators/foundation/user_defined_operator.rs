pub(in crate::combinators) fn user_defined_operator(stream: &mut TokenStream<'_>) -> scan::Result<Box<str>> {
    stream.consume(|tok| {
        let UserDefinedOperator(value) = tok else { return None };
        Some(mem::take(value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_user_defined_op() {
        let mut stream = TokenStream::new("~@", DEFAULT_CONFIG);
        assert_eq!("~@", user_defined_operator(&mut stream).unwrap().as_ref());
    }
}

use crate::scan;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::UserDefinedOperator;
use core::mem;
