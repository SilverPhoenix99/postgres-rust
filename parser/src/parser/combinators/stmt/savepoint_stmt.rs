pub(super) fn savepoint_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
    TransactionStmt:
        SAVEPOINT ColId
    */

    Savepoint
        .and_right(col_id())
        .map(TransactionStmt::Savepoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_savepoint() {
        let mut stream = TokenStream::new("savepoint test_ident", DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Savepoint("test_ident".into())), savepoint_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Savepoint;
use crate::parser::ast_node::TransactionStmt;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
