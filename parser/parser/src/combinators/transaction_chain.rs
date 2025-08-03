/// Alias: `opt_transaction_chain`
pub(super) fn transaction_chain(stream: &mut TokenStream) -> scan::Result<TransactionChain> {

    /*
        AND ( NO )? CHAIN
    */

    let (_, chain, _) = seq!(
        And,
        No.optional(),
        Chain
    ).parse(stream)?;

    let chain = if chain.is_some() {
        TransactionChain::NoChain
    } else {
        TransactionChain::WithChain
    };

    Ok(chain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("and no chain", TransactionChain::NoChain)]
    #[test_case("and chain", TransactionChain::WithChain)]
    fn test_transaction_chain(source: &str, expected: TransactionChain) {
        test_parser!(source, transaction_chain, expected)
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TransactionChain;
use pg_lexer::Keyword::And;
use pg_lexer::Keyword::Chain;
use pg_lexer::Keyword::No;
