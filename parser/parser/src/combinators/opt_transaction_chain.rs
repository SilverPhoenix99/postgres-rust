pub(super) fn opt_transaction_chain() -> impl Combinator<Output = bool> {

    /*
          AND CHAIN
        | AND NO CHAIN
        | /* EMPTY */
    */

    sequence!(
        And.skip(),
        No.optional(),
        Chain.skip()
    ).optional()
        .map(|chain| match chain {
            Some((_, no, _)) => no.is_none(),
            None => false,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("", false)]
    #[test_case("and no chain", false)]
    #[test_case("and chain", true)]
    fn test_opt_transaction_chain(source: &str, expected: bool) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), opt_transaction_chain().parse(&mut stream));
    }
}

use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::And;
use postgres_parser_lexer::Keyword::Chain;
use postgres_parser_lexer::Keyword::No;
