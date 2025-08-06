pub(in crate::combinators::expr::expr_primary)
fn over_clause(stream: &mut TokenStream) -> scan::Result<OverClause> {

    /*
          OVER ColId
        | OVER window_specification
    */

    let (_, expr) = seq!(
        Over,
        alt!(
            col_id.map(WindowName),
            window_specification.map(WindowDefinition)
        )
    ).parse(stream)?;

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::WindowDefinition;
    use test_case::test_case;

    #[test_case("over foo", WindowName("foo".into()))]
    #[test_case("over (foo)",
        OverClause::WindowDefinition(
            WindowDefinition::new(Some("foo".into()), None, None, None)
        )
    )]
    fn test_over_clause(source: &str, expected: OverClause) {
        test_parser!(source, over_clause, expected);
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::window_specification;
use pg_ast::OverClause;
use pg_ast::OverClause::WindowDefinition;
use pg_ast::OverClause::WindowName;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Over;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
