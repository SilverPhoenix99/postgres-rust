pub(in crate::parser::combinators) fn user_defined_operator() -> UserOpCombi {
    UserOpCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser::combinators) struct UserOpCombi;

impl Combinator for UserOpCombi {
    type Output = Box<str>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume(|tok| {
            let UserDefinedOperator(value) = tok else { return None };
            Some(mem::take(value))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;

    #[test]
    fn test_user_defined_op() {
        let mut stream = TokenStream::new("~@", DEFAULT_CONFIG);
        assert_eq!("~@", user_defined_operator().parse(&mut stream).unwrap().as_ref());
    }
}

use crate::parser::combinators::foundation::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::token_value::TokenValue::UserDefinedOperator;
use std::mem;
