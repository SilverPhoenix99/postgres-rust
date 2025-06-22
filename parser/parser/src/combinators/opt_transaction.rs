pub(super) fn opt_transaction(stream: &mut TokenStream) -> Result<()> {

    // Skips over WORK | TRANSACTION

    Work.or(Transaction)
        .optional()
        .skip()
        .parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_opt_transaction() {
        let mut stream = TokenStream::new("transaction work", DEFAULT_CONFIG);
        assert_eq!(Ok(()), opt_transaction(&mut stream));
        assert_eq!(Ok(()), opt_transaction(&mut stream));
        assert_eq!(Ok(()), opt_transaction(&mut stream));
    }
}

use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Work;
