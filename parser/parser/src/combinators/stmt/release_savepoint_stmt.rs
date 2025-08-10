pub(super) fn release_savepoint_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmt:
        RELEASE SAVEPOINT ColId
        RELEASE ColId
    */

    let (.., name) = seq!(Release, Savepoint.optional(), col_id)
        .parse(ctx)?;

    Ok(TransactionStmt::Release(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_release() {
        test_parser!(
            source = "release test_ident",
            parser = release_savepoint_stmt,
            expected = TransactionStmt::Release("test_ident".into())
        )
    }

    #[test]
    fn test_release_savepoint() {
        test_parser!(
            source = "release savepoint test_ident",
            parser = release_savepoint_stmt,
            expected = TransactionStmt::Release("test_ident".into())
        )
    }
}

use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Release;
use pg_lexer::Keyword::Savepoint;
use pg_parser_core::scan;
use pg_sink_combinators::col_id;
use pg_transaction_stmt_ast::TransactionStmt;
