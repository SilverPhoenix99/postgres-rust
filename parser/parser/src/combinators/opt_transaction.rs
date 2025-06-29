pub(super) fn opt_transaction(stream: &mut TokenStream) -> scan::Result<()> {

    // Skips over WORK | TRANSACTION

    or((Work, Transaction))
        .parse(stream)
        .optional()?;

    Ok(())
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

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Work;
