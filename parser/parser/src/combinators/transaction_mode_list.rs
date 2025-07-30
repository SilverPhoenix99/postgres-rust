/// Alias: `transaction_mode_list_or_empty`
pub(super) fn transaction_mode_list(stream: &mut TokenStream) -> scan::Result<Vec<TransactionMode>> {

    /*
        transaction_mode ( (',')? transaction_mode )*
    */

    many_pre(
        transaction_mode,
        or((
            (Comma, transaction_mode)
                .map(|(_, mode)| mode),
            transaction_mode
        ))
    ).parse(stream)
}

/// Alias: `transaction_mode_item`
fn transaction_mode(stream: &mut TokenStream) -> scan::Result<TransactionMode> {

    /*
          ISOLATION LEVEL iso_level
        | READ ONLY
        | READ WRITE
        | DEFERRABLE
        | NOT DEFERRABLE
    */

    or((
        Kw::Deferrable
            .map(|_| Deferrable),
        (Not, Kw::Deferrable)
            .map(|_| NotDeferrable),
        (
            Read,
            or((
                Only.map(|_| ReadOnly),
                Write.map(|_| ReadWrite)
            ))
        )
            .map(|(_, mode)| mode),
        (Isolation, Level, isolation_level)
            .map(|(.., mode)|
                TransactionMode::IsolationLevel(mode)
            )
    )).parse(stream)
}

/// Alias: `iso_level`
fn isolation_level(stream: &mut TokenStream) -> scan::Result<IsolationLevel> {

    /*
          READ UNCOMMITTED
        | READ COMMITTED
        | REPEATABLE READ
        | SERIALIZABLE
    */

    or((
        Kw::Serializable
            .map(|_| Serializable),
        (Repeatable, Read)
            .map(|_| RepeatableRead),
        (
            Read,
            or((
                Committed.map(|_| ReadCommitted),
                Uncommitted.map(|_| ReadUncommitted)
            ))
        )
            .map(|(_, isolation)| isolation)
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use scan::Error::NoMatch;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_transaction_mode_list() {

        let mut stream = TokenStream::new("no_match", DEFAULT_CONFIG);
        assert_matches!(transaction_mode_list(&mut stream), Err(NoMatch(_)));

        let mut stream = TokenStream::new(
            "read only , read write isolation level read committed",
            DEFAULT_CONFIG
        );

        let expected = vec![
            ReadOnly,
            ReadWrite,
            TransactionMode::IsolationLevel(ReadCommitted),
        ];

        assert_eq!(Ok(expected), transaction_mode_list(&mut stream));
    }

    #[test]
    fn test_transaction_mode() {

        let mut stream = TokenStream::new(
            "\
                read only \
                read write \
                deferrable \
                not deferrable \
                isolation level read committed \
                isolation level read uncommitted \
                isolation level repeatable read \
                isolation level serializable\
            ",
            DEFAULT_CONFIG
        );

        let expected = [
            ReadOnly,
            ReadWrite,
            Deferrable,
            NotDeferrable,
            TransactionMode::IsolationLevel(ReadCommitted),
            TransactionMode::IsolationLevel(ReadUncommitted),
            TransactionMode::IsolationLevel(RepeatableRead),
            TransactionMode::IsolationLevel(Serializable),
        ];

        for expected_mode in expected {
            assert_eq!(Ok(expected_mode), transaction_mode(&mut stream));
        }
    }

    #[test]
    fn test_isolation_level() {

        let mut stream = TokenStream::new(
            "\
                read committed \
                read uncommitted \
                repeatable read \
                serializable\
            ",
            DEFAULT_CONFIG
        );

        let expected = [
            ReadCommitted,
            ReadUncommitted,
            RepeatableRead,
            Serializable,
        ];

        for expected_mode in expected {
            assert_eq!(Ok(expected_mode), isolation_level(&mut stream));
        }
    }
}

use crate::combinators::foundation::many_pre;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
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
