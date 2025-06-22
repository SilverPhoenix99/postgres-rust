pub(super) fn abort_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
    TransactionStmt:
        ABORT_P opt_transaction opt_transaction_chain
    */

    Abort
        .and(parser(opt_transaction))
        .and_right(parser(opt_transaction_chain))
        .map(|chain| TransactionStmt::Rollback { chain })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("abort", false)]
    #[test_case("abort and chain", true)]
    #[test_case("abort and no chain", false)]
    #[test_case("abort transaction", false)]
    #[test_case("abort transaction and chain", true)]
    #[test_case("abort transaction and no chain", false)]
    fn test_abort(source: &str, expected: bool) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(TransactionStmt::Rollback { chain: expected }), abort_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::{parser, Combinator};
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use pg_ast::TransactionStmt;
use pg_lexer::Keyword::Abort;
