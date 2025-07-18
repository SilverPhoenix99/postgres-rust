pub(super) fn analyze_keyword(stream: &mut TokenStream) -> scan::Result<()> {

    /*
          ANALYZE
        | ANALYSE
    */

    or((Analyze, Analyse))
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

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Analyse;
use pg_lexer::Keyword::Analyze;
