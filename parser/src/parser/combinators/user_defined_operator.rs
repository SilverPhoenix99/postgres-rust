pub(in crate::parser) fn user_defined_operator() -> UserOpCombi {
    UserOpCombi
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct UserOpCombi;

impl Combinator for UserOpCombi {
    type Output = Box<str>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {
        stream.consume_with_slice(|(tok, slice, _)| match tok {
            UserDefinedOperator => Some(slice.to_string().into_boxed_str()),
            _ => None
        })
    }
}

use crate::lexer::RawTokenKind::UserDefinedOperator;
use crate::parser::combinators::Combinator;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::SlicedTokenConsumer;
use crate::parser::token_stream::TokenStream;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_user_defined_op() {
        let mut stream = TokenStream::new("~@", DEFAULT_CONFIG);
        assert_eq!("~@", user_defined_operator().parse(&mut stream).unwrap().as_ref());
    }
}
