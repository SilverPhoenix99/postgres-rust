pub(super) fn materialized_view(stream: &mut TokenStream) -> Result<QualifiedName> {

    /*
        MATERIALIZED VIEW any_name
    */

    seq!(stream => Materialized, View, any_name)
        .map(|(.., name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_materialized_view() {
        test_parser!(
            source = "materialized view foo",
            parser = materialized_view,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Materialized;
use pg_lexer::Keyword::View;
