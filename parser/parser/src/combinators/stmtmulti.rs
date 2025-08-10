pub(crate) fn stmtmulti(ctx: &mut ParserContext) -> scan::Result<Vec<RawStmt>> {

    // This production is slightly cheating, not because it's more efficient,
    // but helps simplify capturing the combinator.
    // Production:
    //     (';')* ( toplevel_stmt ( (';')+ toplevel_stmt? )* )?
    // Original production:
    //     toplevel_stmt? ( ';' toplevel_stmt? )*

    let (_, stmts) = seq!(
        semicolons.optional(),
        many!(sep = semicolons, toplevel_stmt).optional()
    ).parse(ctx)?;

    // Note that `many` returns `NoMatch` if the Vec would be empty.
    match stmts {
        Some(stmts) => Ok(stmts),
        None if ctx.stream_mut().eof() => {
            // The content didn't have any statements.
            // The stream is either empty, or whitespaces and/or semicolons.
            let loc = ctx.stream_mut().current_location();
            Err(Eof(loc))
        },
        None => {
            // It's not Eof, so there was a syntax error.
            no_match(ctx)
        },
    }
}

/// Returns `Ok` if it consumed at least 1 `;` (semicolon).
fn semicolons(ctx: &mut ParserContext) -> scan::Result<()> {

    // Production: ( ';' )+

    // skip() might look unnecessary, but it makes the elements have 0 bytes,
    // so the Vec never allocates.

    many!(Semicolon.skip()).parse(ctx)?;

    Ok(())
}

fn toplevel_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    alt!(
        transaction_stmt_legacy.map(RawStmt::from),
        stmt
    ).parse(ctx)
}

/// Alias: `TransactionStmtLegacy`
fn transaction_stmt_legacy(ctx: &mut ParserContext) -> scan::Result<TransactionStmt> {

    alt!(
        begin_stmt,
        end_stmt
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    #[allow(unused_imports)]
    use pg_transaction_stmt_ast::TransactionMode::ReadOnly;
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

    #[test_case("begin transaction read only" => Ok(TransactionStmt::Begin(vec![ReadOnly])))]
    #[test_case("end transaction" => Ok(TransactionStmt::Commit { chain: false }))]
    fn test_transaction(source: &str) -> scan::Result<TransactionStmt> {
        test_parser!(source, transaction_stmt_legacy)
    }
}

use crate::combinators::stmt;
use crate::combinators::stmt::begin_stmt;
use crate::combinators::stmt::end_stmt;
use crate::no_match;
use pg_ast::RawStmt;
use pg_combinators::alt;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::OperatorKind::Semicolon;
use pg_parser_core::scan;
use pg_parser_core::scan::Error::Eof;
use pg_transaction_stmt_ast::TransactionStmt;
