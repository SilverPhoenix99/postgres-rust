pub(super) fn materialized_view() -> impl Combinator<Output = QualifiedName> {

    /*
        MATERIALIZED VIEW any_name
    */

    and(Materialized, View)
        .and_right(any_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_materialized_view() {
        test_parser!(
            source = "materialized view foo",
            parser = materialized_view(),
            expected = vec!["foo".into()]
        )
    }
}

use crate::lexer::Keyword::Materialized;
use crate::lexer::Keyword::View;
use crate::parser::ast_node::QualifiedName;
use crate::parser::combinators::any_name;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
