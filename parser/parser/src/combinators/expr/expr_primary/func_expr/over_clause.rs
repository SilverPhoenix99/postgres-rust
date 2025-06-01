pub(super) fn over_clause() -> impl Combinator<Output = Option<OverClause>> {

    /*
          OVER ColId
        | OVER window_specification
    */

    Over.and_right(or(
        col_id().map(WindowName),
        window_specification().map(WindowDefinition),
    ))
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::WindowDefinition;
    use test_case::test_case;

    #[test_case("over foo", Some(WindowName("foo".into())))]
    #[test_case("over (foo)", Some(
        OverClause::WindowDefinition(
            WindowDefinition::new(Some("foo".into()), None, None, None)
        )
    ))]
    fn test_over_clause(source: &str, expected: Option<OverClause>) {
        test_parser!(source, over_clause(), expected);
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::window_specification::window_specification;
use pg_ast::OverClause;
use pg_ast::OverClause::WindowDefinition;
use pg_ast::OverClause::WindowName;
use pg_lexer::Keyword::Over;
