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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("", false)]
    #[test_case("and no chain", false)]
    #[test_case("and chain", true)]
    fn test_opt_transaction_chain(source: &str, expected: bool) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), opt_transaction_chain().parse(&mut stream));
    }
}

use crate::lexer::Keyword::And;
use crate::lexer::Keyword::Chain;
use crate::lexer::Keyword::No;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
