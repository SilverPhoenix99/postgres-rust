pub(super) fn opt_transaction() -> impl Combinator<Output = ()> {

    // Skips over WORK | TRANSACTION

    Work.or(Transaction)
        .optional()
        .skip()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_opt_transaction() {
        let mut stream = TokenStream::new("transaction work", DEFAULT_CONFIG);
        assert_eq!(Ok(()), opt_transaction().parse(&mut stream));
        assert_eq!(Ok(()), opt_transaction().parse(&mut stream));
        assert_eq!(Ok(()), opt_transaction().parse(&mut stream));
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Transaction;
use postgres_parser_lexer::Keyword::Work;
