/// Post-condition: Vec is **Not** empty
///
/// Alias: `transaction_mode_list_or_empty`
pub(super) fn transaction_mode_list() -> impl Combinator<Output = Vec<TransactionMode>> {

    /*
        transaction_mode ( (',')? transaction_mode )*
    */

    enclosure! {
        many_pre(
            transaction_mode(),
            parser(|stream| {
                let result = Comma.parse(stream).optional()?;
                if result.is_some() {
                    transaction_mode().required()
                        .parse(stream)
                }
                else {
                    transaction_mode()
                        .parse(stream)
                }
            })
        )
    }
}

/// Alias: `transaction_mode_item`
fn transaction_mode() -> impl Combinator<Output = TransactionMode> {
    use Keyword::{self as Kw, Isolation, Level, Not, Only, Read, Write};

    /*
          ISOLATION LEVEL iso_level
        | READ ONLY
        | READ WRITE
        | DEFERRABLE
        | NOT DEFERRABLE
    */

    match_first!{
        Kw::Deferrable.map(|_| Deferrable),
        Not.and_then(Kw::Deferrable, |_, _| NotDeferrable),
        Read.and_right(
            match_first!{
                Only.map(|_| ReadOnly),
                Write.map(|_| ReadWrite)
            }
        ),
        Isolation.and(Level)
            .and_right(isolation_level())
            .map(TransactionMode::IsolationLevel)
    }
}

/// Alias: `iso_level`
fn isolation_level() -> impl Combinator<Output = IsolationLevel> {
    use Keyword::{Committed, Read, Repeatable, Serializable, Uncommitted};

    /*
          READ UNCOMMITTED
        | READ COMMITTED
        | REPEATABLE READ
        | SERIALIZABLE
    */

    match_first!{
        Serializable.map(|_| IsolationLevel::Serializable),
        Repeatable
            .and_then(Read, |_, _| RepeatableRead),
        Read.and_right(
            match_first!{
                Committed.map(|_| ReadCommitted),
                Uncommitted.map(|_| ReadUncommitted)
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::result::ScanErrorKind::NoMatch;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use TransactionMode::ReadOnly;
    use TransactionMode::ReadWrite;

    #[test]
    fn test_opt_transaction_mode_list() {

        let mut stream = TokenStream::new("no_match", DEFAULT_CONFIG);
        assert_matches!(transaction_mode_list().parse(&mut stream), Err(NoMatch(_)));

        let mut stream = TokenStream::new(
            "read only , read write isolation level read committed",
            DEFAULT_CONFIG
        );

        let expected = vec![
            ReadOnly,
            ReadWrite,
            TransactionMode::IsolationLevel(ReadCommitted),
        ];

        assert_eq!(Ok(expected), transaction_mode_list().parse(&mut stream));
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
            assert_eq!(Ok(expected_mode), transaction_mode().parse(&mut stream));
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
            assert_eq!(Ok(expected_mode), isolation_level().parse(&mut stream));
        }
    }
}

use crate::lexer::Keyword;
use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::IsolationLevel;
use crate::parser::ast_node::IsolationLevel::ReadCommitted;
use crate::parser::ast_node::IsolationLevel::ReadUncommitted;
use crate::parser::ast_node::IsolationLevel::RepeatableRead;
use crate::parser::ast_node::TransactionMode;
use crate::parser::ast_node::TransactionMode::Deferrable;
use crate::parser::ast_node::TransactionMode::NotDeferrable;
use crate::parser::ast_node::TransactionMode::ReadOnly;
use crate::parser::ast_node::TransactionMode::ReadWrite;
use crate::parser::combinators::foundation::enclosure;
use crate::parser::combinators::foundation::many_pre;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::parser;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::result::Optional;
