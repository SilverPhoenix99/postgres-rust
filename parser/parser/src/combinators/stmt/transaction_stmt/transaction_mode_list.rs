/// Alias: `transaction_mode_list_or_empty`
pub(in crate::combinators::stmt) fn transaction_mode_list(ctx: &mut ParserContext) -> scan::Result<Vec<TransactionMode>> {

    /*
        transaction_mode ( (',')? transaction_mode )*
    */

    many!(
        pre = transaction_mode,
        alt!(
            seq!(Comma, transaction_mode)
                .map(|(_, mode)| mode),
            transaction_mode
        )
    ).parse(ctx)
}

/// Alias: `transaction_mode_item`
fn transaction_mode(ctx: &mut ParserContext) -> scan::Result<TransactionMode> {

    /*
          ISOLATION LEVEL iso_level
        | READ ONLY
        | READ WRITE
        | DEFERRABLE
        | NOT DEFERRABLE
    */

    alt!(
        Kw::Deferrable
            .map(|_| Deferrable),
        seq!(Not, Kw::Deferrable)
            .map(|_| NotDeferrable),
        seq!(
            Read,
            alt!(
                Only.map(|_| ReadOnly),
                Write.map(|_| ReadWrite)
            )
        )
            .map(|(_, mode)| mode),
        seq!(Isolation, Level, isolation_level)
            .map(|(.., mode)|
                TransactionMode::IsolationLevel(mode)
            )
    ).parse(ctx)
}

/// Alias: `iso_level`
fn isolation_level(ctx: &mut ParserContext) -> scan::Result<IsolationLevel> {

    /*
          READ UNCOMMITTED
        | READ COMMITTED
        | REPEATABLE READ
        | SERIALIZABLE
    */

    alt!(
        Kw::Serializable
            .map(|_| Serializable),
        seq!(Repeatable, Read)
            .map(|_| RepeatableRead),
        seq!(
            Read,
            alt!(
                Committed.map(|_| ReadCommitted),
                Uncommitted.map(|_| ReadUncommitted)
            )
        )
            .map(|(_, isolation)| isolation)
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use scan::Error::NoMatch;
    use test_case::test_matrix;

    #[test_matrix("read only , read write isolation level read committed" => Ok(
        vec![
            ReadOnly,
            ReadWrite,
            TransactionMode::IsolationLevel(ReadCommitted),
        ]
    ))]
    #[test_matrix("no_match" => matches Err(NoMatch(_)))]
    fn test_transaction_mode_list(source: &str) -> scan::Result<Vec<TransactionMode>> {
        test_parser!(source, transaction_mode_list)
    }

    #[test_matrix("read only" => Ok(ReadOnly))]
    #[test_matrix("read write" => Ok(ReadWrite))]
    #[test_matrix("deferrable" => Ok(Deferrable))]
    #[test_matrix("not deferrable" => Ok(NotDeferrable))]
    #[test_matrix("isolation level read committed" => Ok(TransactionMode::IsolationLevel(ReadCommitted)))]
    #[test_matrix("isolation level read uncommitted" => Ok(TransactionMode::IsolationLevel(ReadUncommitted)))]
    #[test_matrix("isolation level repeatable read" => Ok(TransactionMode::IsolationLevel(RepeatableRead)))]
    #[test_matrix("isolation level serializable" => Ok(TransactionMode::IsolationLevel(Serializable)))]
    fn test_transaction_mode(source: &str) -> scan::Result<TransactionMode> {
        test_parser!(source, transaction_mode)
    }

    #[test_matrix("read committed" => Ok(ReadCommitted))]
    #[test_matrix("read uncommitted" => Ok(ReadUncommitted))]
    #[test_matrix("repeatable read" => Ok(RepeatableRead))]
    #[test_matrix("serializable" => Ok(Serializable))]
    fn test_isolation_level(source: &str) -> scan::Result<IsolationLevel> {
        test_parser!(source, isolation_level)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::IsolationLevel;
use pg_ast::IsolationLevel::ReadCommitted;
use pg_ast::IsolationLevel::ReadUncommitted;
use pg_ast::IsolationLevel::RepeatableRead;
use pg_ast::IsolationLevel::Serializable;
use pg_ast::TransactionMode;
use pg_ast::TransactionMode::Deferrable;
use pg_ast::TransactionMode::NotDeferrable;
use pg_ast::TransactionMode::ReadOnly;
use pg_ast::TransactionMode::ReadWrite;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Committed;
use pg_lexer::Keyword::Isolation;
use pg_lexer::Keyword::Level;
use pg_lexer::Keyword::Not;
use pg_lexer::Keyword::Only;
use pg_lexer::Keyword::Read;
use pg_lexer::Keyword::Repeatable;
use pg_lexer::Keyword::Uncommitted;
use pg_lexer::Keyword::Write;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
