/// Alias: `opt_transaction`
pub fn work_or_transaction(ctx: &mut ParserContext) -> scan::Result<()> {

    // Skips over WORK | TRANSACTION

    alt!(Work, Transaction)
        .parse(ctx)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_or_transaction() {
        let mut ctx = ParserContext::from("transaction work");
        assert_eq!(Ok(()), work_or_transaction(&mut ctx));
        assert_eq!(Ok(()), work_or_transaction(&mut ctx));
    }
}

use pg_combinators::alt;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Transaction;
use pg_lexer::Keyword::Work;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
