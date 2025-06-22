/// Alias: `transaction_mode_list_or_empty`
pub(super) fn transaction_mode_list(stream: &mut TokenStream) -> Result<Vec<TransactionMode>> {

    /*
        transaction_mode ( (',')? transaction_mode )*
    */

    many!(
        pre = transaction_mode,
        choice!(
            seq!(Comma, transaction_mode).right(),
            transaction_mode
        )
    )
        .parse(stream)
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

    choice!(
        Kw::Deferrable.map(|_| Deferrable),
        Not.and_then(Kw::Deferrable, |_, _| NotDeferrable),
        seq!(
            Read,
            choice!(
                Only.map(|_| ReadOnly),
                Write.map(|_| ReadWrite)
            )
        )
            .right::<_, TransactionMode>(),
        seq!(Isolation, Level, isolation_level)
            .map(|(.., mode)| mode)
            .map(TransactionMode::IsolationLevel)
    )
        .parse(stream)
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

    choice!(
        Serializable.map(|_| IsolationLevel::Serializable),
        Repeatable
            .and_then(Read, |_, _| RepeatableRead),
        seq!(
            Read,
            choice!(
                Committed.map(|_| ReadCommitted),
                Uncommitted.map(|_| ReadUncommitted)
            )
        )
        .right::<_, IsolationLevel>()
    )
        .parse(stream)
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
