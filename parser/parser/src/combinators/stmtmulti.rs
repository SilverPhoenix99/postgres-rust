pub(crate) fn stmtmulti() -> impl Combinator<Output = Vec<RawStmt>> {

    // This production is slightly cheating, not because it's more efficient,
    // but helps simplify capturing the combinator.
    // Production:
    //     (';')* ( toplevel_stmt ( (';')+ toplevel_stmt? )* )?
    // Original production:
    //     toplevel_stmt? ( ';' toplevel_stmt? )*

    semicolons().optional()
        .and_right(
            many_sep(semicolons(), toplevel_stmt()).optional()
        )
        .map(|stmts|
            stmts.into_iter()
                .flatten()
                .collect()
        )
}

/// Returns `Ok` if it consumed at least 1 `;` (semicolon).
fn semicolons() -> impl Combinator<Output = ()> {

    // Production: ( ';' )+

    many(Semicolon.skip()).skip()
}

fn toplevel_stmt() -> impl Combinator<Output = RawStmt> {
    match_first!(
        transaction_stmt_legacy().map(From::from),
        stmt()
    )
}

/// Alias: `TransactionStmtLegacy`
fn transaction_stmt_legacy() -> impl Combinator<Output = TransactionStmt> {
    match_first!{
        begin_stmt(),
        end_stmt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    #[allow(unused_imports)]
    use postgres_parser_ast::TransactionMode::ReadOnly;
    use test_case::test_case;

    #[test_case("begin transaction")]
    #[test_case("start transaction")]
    #[test_case("end transaction")]
    fn test_toplevel_stmt(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = toplevel_stmt().parse(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }

    #[test_case("begin transaction read only", TransactionStmt::Begin(vec![ReadOnly]))]
    #[test_case("end transaction", TransactionStmt::Commit { chain: false })]
    fn test_transaction(source: &str, expected: TransactionStmt) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = transaction_stmt_legacy().parse(&mut stream);
        assert_eq!(Ok(expected), actual);
    }
}

use crate::combinators::foundation::many;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::stmt;
use crate::combinators::stmt::begin_stmt;
use crate::combinators::stmt::end_stmt;
use postgres_parser_ast::RawStmt;
use postgres_parser_ast::TransactionStmt;
use postgres_parser_lexer::OperatorKind::Semicolon;
