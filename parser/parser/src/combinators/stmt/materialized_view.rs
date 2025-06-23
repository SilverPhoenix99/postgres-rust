pub(super) fn materialized_view() -> impl Combinator<Output = QualifiedName> {

    /*
        MATERIALIZED VIEW any_name
    */

    (Materialized, View)
        .and_right(any_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_materialized_view() {
        test_parser!(
            source = "materialized view foo",
            parser = materialized_view(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Materialized;
use pg_lexer::Keyword::View;
