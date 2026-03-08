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
    use crate::test_parser;

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

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Release;
use pg_lexer::Keyword::Savepoint;
use pg_parser_core::scan;
