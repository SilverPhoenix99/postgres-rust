/// Alias: `transaction_mode_list_or_empty`
pub(super) fn transaction_mode_list(stream: &mut TokenStream) -> Result<Vec<TransactionMode>> {

    /*
        transaction_mode ( (',')? transaction_mode )*
    */

    many!(=>
        pre = transaction_mode.parse(stream),
        choice!(stream =>
            seq!(stream => Comma.skip(), transaction_mode)
                .map(|(_, mode)| mode),
            transaction_mode.parse(stream)
        )
    )
}

/// Alias: `transaction_mode_item`
fn transaction_mode(stream: &mut TokenStream) -> Result<TransactionMode> {
    use Keyword::{self as Kw, Isolation, Level, Not, Only, Read, Write};

    /*
          ISOLATION LEVEL iso_level
        | READ ONLY
        | READ WRITE
        | DEFERRABLE
        | NOT DEFERRABLE
    */

    choice!(stream =>
        Kw::Deferrable
            .parse(stream)
            .map(|_| Deferrable),
        seq!(stream => Not, Kw::Deferrable )
            .map(|_| NotDeferrable),
        seq!(=> 
            Read.parse(stream),
            choice!(stream =>
                Only.parse(stream).map(|_| ReadOnly),
                Write.parse(stream).map(|_| ReadWrite)
            )
        )
            .map(|(_, mode)| mode),
        seq!(stream => Isolation, Level, isolation_level)
            .map(|(.., mode)|
                TransactionMode::IsolationLevel(mode)
            )
    )
}

/// Alias: `iso_level`
fn isolation_level(stream: &mut TokenStream) -> Result<IsolationLevel> {
    use Keyword::{Committed, Read, Repeatable, Serializable, Uncommitted};

    /*
          READ UNCOMMITTED
        | READ COMMITTED
        | REPEATABLE READ
        | SERIALIZABLE
    */

    choice!(stream =>
        Serializable
            .parse(stream)
            .map(|_| IsolationLevel::Serializable),
        seq!(stream => Repeatable, Read)
            .map(|_| RepeatableRead),
        seq!(=>
            Read.parse(stream),
            choice!(stream =>
                Committed.parse(stream).map(|_| ReadCommitted),
                Uncommitted.parse(stream).map(|_| ReadUncommitted)
            )
        )
        .map(|(_, isolation)| isolation)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Error::NoMatch;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_opt_transaction_mode_list() {

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
            TransactionMode::IsolationLevel(IsolationLevel::Serializable),
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
            IsolationLevel::Serializable,
        ];

        for expected_mode in expected {
            assert_eq!(Ok(expected_mode), isolation_level(&mut stream));
        }
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::IsolationLevel;
use pg_ast::IsolationLevel::ReadCommitted;
use pg_ast::IsolationLevel::ReadUncommitted;
use pg_ast::IsolationLevel::RepeatableRead;
use pg_ast::TransactionMode;
use pg_ast::TransactionMode::Deferrable;
use pg_ast::TransactionMode::NotDeferrable;
use pg_ast::TransactionMode::ReadOnly;
use pg_ast::TransactionMode::ReadWrite;
use pg_lexer::Keyword;
use pg_lexer::OperatorKind::Comma;
