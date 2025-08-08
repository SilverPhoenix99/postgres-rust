pub fn materialized_view(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        MATERIALIZED VIEW any_name
    */

    let (.., name) = seq!(Materialized, View, any_name)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_materialized_view() {
        test_parser!(
            source = "materialized view foo",
            parser = materialized_view,
            expected = vec!["foo".into()]
        )
    }
}

use crate::any_name;
use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Materialized;
use pg_lexer::Keyword::View;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
