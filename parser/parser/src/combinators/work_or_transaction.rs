pub(super) fn work_or_transaction(stream: &mut TokenStream) -> scan::Result<()> {

    // Skips over WORK | TRANSACTION

    alt!(Work, Transaction)
        .parse(stream)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;

    #[test]
    fn test_work_or_transaction() {
        let mut stream = TokenStream::from("transaction work");
        assert_eq!(Ok(()), work_or_transaction(&mut stream));
        assert_eq!(Ok(()), work_or_transaction(&mut stream));
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Work;
use pg_parser_core::scan;
