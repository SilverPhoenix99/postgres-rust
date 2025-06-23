use crate::result::Optional;
pub(crate) fn stmtmulti(stream: &mut TokenStream) -> Result<Option<Vec<RawStmt>>> {

    // This production is slightly cheating, not because it's more efficient,
    // but helps simplify capturing the combinator.
    // Production:
    //     (';')* ( toplevel_stmt ( (';')+ toplevel_stmt? )* )?
    // Original production:
    //     toplevel_stmt? ( ';' toplevel_stmt? )*
    
    seq!(=>
        semicolons.parse(stream).optional(),
        many!(=>
            sep = semicolons.parse(stream),
            toplevel_stmt.parse(stream)
        ).optional()
    )
        .map(|(_, stmts)| stmts)
}

/// Returns `Ok` if it consumed at least 1 `;` (semicolon).
fn semicolons(stream: &mut TokenStream) -> Result<()> {

    // Production: ( ';' )+

    many!(Semicolon.skip())
        .skip()
        .parse(stream)
}

fn toplevel_stmt(stream: &mut TokenStream) -> Result<RawStmt> {

    choice!(
        transaction_stmt_legacy,
        stmt
    )
        .parse(stream)
}

/// Alias: `TransactionStmtLegacy`
fn transaction_stmt_legacy(stream: &mut TokenStream) -> Result<TransactionStmt> {

    choice!(begin_stmt, end_stmt)
        .parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::{test_parser, DEFAULT_CONFIG};
    #[allow(unused_imports)]
    use pg_ast::TransactionMode::ReadOnly;
    use test_case::test_case;

    #[test_case("begin transaction")]
    #[test_case("start transaction")]
    #[test_case("end transaction")]
    fn test_toplevel_stmt(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = toplevel_stmt(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }

    #[test_case("begin transaction read only", TransactionStmt::Begin(vec![ReadOnly]))]
    #[test_case("end transaction", TransactionStmt::Commit { chain: false })]
    fn test_transaction(source: &str, expected: TransactionStmt) {
        test_parser!(source, transaction_stmt_legacy, expected)
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::stmt;
use crate::combinators::stmt::begin_stmt;
use crate::combinators::stmt::end_stmt;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_ast::TransactionStmt;
use pg_lexer::OperatorKind::Semicolon;
