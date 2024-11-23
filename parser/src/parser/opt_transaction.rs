pub(super) fn opt_transaction() -> impl Combinator<Output = ()> {
    use crate::lexer::Keyword::{Transaction, Work};
    use crate::parser::combinators::{keyword_if, CombinatorHelpers};

    // Skips over WORK | TRANSACTION

    keyword_if(|kw| matches!(kw, Work | Transaction))
        .optional()
        .skip()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_opt_transaction() {
        let mut stream = TokenStream::new("transaction work", DEFAULT_CONFIG);
        assert_eq!(Ok(()), opt_transaction().parse(&mut stream));
        assert_eq!(Ok(()), opt_transaction().parse(&mut stream));
        assert_eq!(Ok(()), opt_transaction().parse(&mut stream));
    }
}

use crate::parser::combinators::Combinator;
