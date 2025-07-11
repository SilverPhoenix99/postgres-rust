pub(super) fn savepoint_stmt(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    /*
    TransactionStmt:
        SAVEPOINT ColId
    */

    let (_, name) = (Savepoint, col_id)
        .parse(stream)?;

    let stmt = TransactionStmt::Savepoint(name);

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Savepoint;
