pub(super) fn opt_transaction_chain(stream: &mut TokenStream) -> Result<bool> {

    /*
          AND CHAIN
        | AND NO CHAIN
        | /* EMPTY */
    */

    sequence!(
        And.skip(),
        No.optional(),
        Chain.skip()
    )
        .optional()
        .map(|chain| match chain {
            Some((_, no, _)) => no.is_none(),
            None => false,
        })
        .parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::foundation::ClosureHelpers;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("", false)]
    #[test_case("and no chain", false)]
    #[test_case("and chain", true)]
    fn test_opt_transaction_chain(source: &str, expected: bool) {
        test_parser!(source, opt_transaction_chain, expected)
    }
}

use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_lexer::Keyword::And;
use pg_lexer::Keyword::Chain;
use pg_lexer::Keyword::No;
