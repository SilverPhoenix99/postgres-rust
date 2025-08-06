pub(crate) fn stmtmulti(stream: &mut TokenStream) -> scan::Result<Vec<RawStmt>> {

    // This production is slightly cheating, not because it's more efficient,
    // but helps simplify capturing the combinator.
    // Production:
    //     (';')* ( toplevel_stmt ( (';')+ toplevel_stmt? )* )?
    // Original production:
    //     toplevel_stmt? ( ';' toplevel_stmt? )*

    let (_, stmts) = seq!(
        semicolons.optional(),
        many!(sep = semicolons, toplevel_stmt).optional()
    ).parse(stream)?;

    // Note that `many` returns `NoMatch` if the Vec would be empty.
    match stmts {
        Some(stmts) => Ok(stmts),
        None if stream.eof() => {
            // The content didn't have any statements.
            // The stream is either empty, or whitespaces and/or semicolons.
            let loc = stream.current_location();
            Err(Eof(loc))
        },
        None => {
            // It's not Eof, so there was a syntax error.
            no_match(stream)
        },
    }
}

/// Returns `Ok` if it consumed at least 1 `;` (semicolon).
fn semicolons(stream: &mut TokenStream) -> scan::Result<()> {

    // Production: ( ';' )+

    // skip() might look unnecessary, but it makes the elements have 0 bytes,
    // so the Vec never allocates.

    many!(Semicolon.skip()).parse(stream)?;

    Ok(())
}

fn toplevel_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    alt!(
        transaction_stmt_legacy.map(RawStmt::from),
        stmt
    ).parse(stream)
}

/// Alias: `TransactionStmtLegacy`
fn transaction_stmt_legacy(stream: &mut TokenStream) -> scan::Result<TransactionStmt> {

    alt!(
        begin_stmt,
        end_stmt
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        TransactionChain::NoChain,
        TransactionMode::ReadOnly
    };
    use test_case::test_case;
    use test_case::test_matrix;

    // This only quickly tests that statement types aren't missing.
    // More in-depth testing is within each statement's module.
    #[test_matrix(
        [
            "begin transaction",
            "start transaction",
            "end transaction",
        ]
        => matches Ok(_)
    )]
    fn test_toplevel_stmt(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, toplevel_stmt)
    }

    #[test_case("begin transaction read only", TransactionStmt::Begin(vec![ReadOnly]))]
    #[test_case("end transaction", TransactionStmt::Commit(NoChain))]
    fn test_transaction(source: &str, expected: TransactionStmt) {
        test_parser!(source, transaction_stmt_legacy, expected)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::stmt;
use crate::combinators::stmt::begin_stmt;
use crate::combinators::stmt::end_stmt;
use crate::no_match;
use pg_ast::RawStmt;
use pg_ast::TransactionStmt;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Semicolon;
use pg_parser_core::scan;
use pg_parser_core::scan::Error::Eof;
use pg_parser_core::stream::TokenStream;
