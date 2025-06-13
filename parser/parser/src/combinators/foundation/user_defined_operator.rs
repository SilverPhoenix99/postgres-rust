pub(in crate::combinators) fn user_defined_operator() -> UserOpCombi {
    UserOpCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combinators) struct UserOpCombi;

impl Combinator for UserOpCombi {
    type Output = Box<str>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output> {
        stream.consume(|tok| {
            let UserDefinedOperator(value) = tok else { return None };
            Some(mem::take(value))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_user_defined_op() {
        let mut stream = TokenStream::new("~@", DEFAULT_CONFIG);
        assert_eq!("~@", user_defined_operator().parse(&mut stream).unwrap().as_ref());
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenConsumer;
use crate::stream::TokenStream;
use crate::stream::TokenValue::UserDefinedOperator;
use std::mem;
