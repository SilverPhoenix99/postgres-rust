/// Alias: `opt_analyze`
pub(super) fn analyze_keyword(stream: &mut TokenStream) -> scan::Result<()> {

    /*
          ANALYZE
        | ANALYSE
    */

    alt!(Analyze, Analyse)
        .parse(stream)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("analyze")]
    #[test_case("analyse")]
    fn test_analyze_keyword(source: &str) {
        test_parser!(source, analyze_keyword, ())
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
use pg_lexer::Keyword::Analyse;
use pg_lexer::Keyword::Analyze;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
