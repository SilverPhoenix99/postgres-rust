
pub(super) fn opt_transaction(stream: &mut TokenStream) -> Result<()> {

    // Skips over WORK | TRANSACTION

    choice!(stream =>
        Work.parse(stream),
        Transaction.parse(stream)
    )
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

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::choice;
use crate::result::Optional;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Work;
