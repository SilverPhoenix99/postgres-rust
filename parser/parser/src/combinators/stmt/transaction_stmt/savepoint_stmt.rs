pub(super) fn savepoint_stmt(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmt:
        SAVEPOINT ColId
    */

    let (_, name) = seq!(Savepoint, col_id)
        .parse(ctx)?;

    let stmt = TransactionStmt::Savepoint(name);

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_savepoint() {
        test_parser!(
            source = "savepoint test_ident",
            parser = savepoint_stmt,
            expected = TransactionStmt::Savepoint("test_ident".into())
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Savepoint;
use pg_parser_core::scan;
