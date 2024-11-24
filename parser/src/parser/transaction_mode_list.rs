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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::IsolationLevel::ReadCommitted;
    use crate::parser::result::ScanErrorKind::NoMatch;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_opt_transaction_mode_list() {

        let mut stream = TokenStream::new("no_match", DEFAULT_CONFIG);
        assert_matches!(transaction_mode_list().parse(&mut stream), Err(NoMatch(_)));

        let mut stream = TokenStream::new(
            "read only , read write isolation level read committed",
            DEFAULT_CONFIG
        );

        let expected = vec![
            TransactionMode::ReadOnly,
            TransactionMode::ReadWrite,
            TransactionMode::IsolationLevel(ReadCommitted),
        ];

        assert_eq!(Ok(expected), transaction_mode_list().parse(&mut stream));
    }
}

use crate::lexer::OperatorKind::Comma;
use crate::parser::ast_node::TransactionMode;
use crate::parser::combinators::enclosure;
use crate::parser::combinators::many_pre;
use crate::parser::combinators::parser;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::result::Optional;
use crate::parser::transaction_mode;
