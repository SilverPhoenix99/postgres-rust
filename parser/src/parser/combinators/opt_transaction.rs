pub(super) fn opt_transaction() -> impl Combinator<Output = ()> {

    // Skips over WORK | TRANSACTION

    Work.or(Transaction)
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

use crate::lexer::Keyword::Transaction;
use crate::lexer::Keyword::Work;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
