pub(super) fn statistics(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        STATISTICS any_name
    */

    let (_, name) = seq!(Statistics, any_name)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_statistics() {
        test_parser!(
            source = "statistics foo",
            parser = statistics,
            expected = vec!["foo".into()]
        )
    }
}

use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_identifier_combinators::any_name;
use pg_lexer::Keyword::Statistics;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
