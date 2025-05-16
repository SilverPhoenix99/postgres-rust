pub(in crate::combinators) fn end_stmt() -> impl Combinator<Output = TransactionStmt> {

    /*
    TransactionStmtLegacy:
        END_P opt_transaction opt_transaction_chain
    */

    End
        .and(opt_transaction())
        .and_right(opt_transaction_chain())
        .map(|chain| Commit { chain })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("end", false)]
    #[test_case("end and chain", true)]
    #[test_case("end and no chain", false)]
    #[test_case("end transaction", false)]
    #[test_case("end transaction and chain", true)]
    #[test_case("end transaction and no chain", false)]
    fn test_end(source: &str, expected: bool) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(Commit { chain: expected }), end_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::opt_transaction;
use crate::combinators::opt_transaction_chain;
use pg_ast::TransactionStmt;
use pg_ast::TransactionStmt::Commit;
use pg_lexer::Keyword::End;
