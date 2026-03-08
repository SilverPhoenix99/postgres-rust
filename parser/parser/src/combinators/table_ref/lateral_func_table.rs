/// Alias: `opt_ordinality`
fn ordinality(ctx: &mut ParserContext) -> scan::Result<()> {

    /*
        WITH ORDINALITY
    */

    seq!(With, Ordinality).parse(ctx)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("with ordinality" => Ok(()))]
    fn test_ordinality(source: &str) -> scan::Result<()> {
        test_parser!(source, ordinality)
    }
}

use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_lexer::Keyword::Ordinality;
use pg_lexer::Keyword::With;
use pg_parser_core::scan;
