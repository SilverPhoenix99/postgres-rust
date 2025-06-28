pub(super) fn over_clause(stream: &mut TokenStream) -> scan::Result<Option<OverClause>> {

    /*
          OVER ColId
        | OVER window_specification
    */

    let expr = seq!(=>
        Over.parse(stream),
        choice!(parsed stream =>
            col_id.map(WindowName),
            window_specification.map(WindowDefinition)
        )
    );

    let expr = expr.map(|(_, expr)| expr).optional()?;
    Ok(expr)
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
        test_parser!(source, over_clause, expected);
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::window_specification;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::OverClause;
use pg_ast::OverClause::WindowDefinition;
use pg_ast::OverClause::WindowName;
use pg_lexer::Keyword::Over;
