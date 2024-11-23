pub(super) fn opt_transaction_chain() -> impl Combinator<Output = bool> {
    use crate::lexer::Keyword::{And, Chain, No};
    use crate::parser::combinators::{keyword, CombinatorHelpers};

    /*
          AND CHAIN
        | AND NO CHAIN
        | /* EMPTY */
    */

    enclosure! {
        keyword(And)
            .and_right(
                keyword(No)
                    .optional()
                    .map(|no| no.is_none())
            )
            .and_left(keyword(Chain))
            .optional()
            .map(|chain| chain.unwrap_or(false))
    }
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

use crate::parser::combinators::enclosure;
use crate::parser::combinators::Combinator;
