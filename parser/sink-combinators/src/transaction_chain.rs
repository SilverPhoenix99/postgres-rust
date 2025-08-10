/// Alias: `opt_transaction_chain`
pub fn transaction_chain(ctx: &mut ParserContext) -> scan::Result<bool> {

    /*
        AND ( NO )? CHAIN
    */

    let (_, no, _) = seq!(
        And,
        No.optional(),
        Chain
    ).parse(ctx)?;

    Ok(no.is_none())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("and no chain" => Ok(false))]
    #[test_case("and chain" => Ok(true))]
    fn test_transaction_chain(source: &str) -> scan::Result<bool> {
        test_parser!(source, transaction_chain)
    }
}

use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::And;
use pg_lexer::Keyword::Chain;
use pg_lexer::Keyword::No;
use pg_parser_core::scan;
