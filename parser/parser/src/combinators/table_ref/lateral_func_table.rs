/// Alias: `opt_ordinality`
fn ordinality(stream: &mut TokenStream) -> scan::Result<()> {

    /*
        WITH ORDINALITY
    */

    seq!(With, Ordinality).parse(stream)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("with ordinality" => Ok(()))]
    fn test_ordinality(source: &str) -> scan::Result<()> {
        test_parser!(source, ordinality)
    }
}

use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Ordinality;
use pg_lexer::Keyword::With;
use pg_parser_core::scan;
