/// Alias: `opt_window_exclusion_clause`
pub(super) fn window_exclusion_clause(ctx: &mut ParserContext) -> scan::Result<WindowExclusion> {

    /*
          EXCLUDE CURRENT ROW
        | EXCLUDE GROUP
        | EXCLUDE TIES
        | EXCLUDE NO OTHERS
    */

    let (_, exclusion) = seq!(
        Exclude,
        alt!(
            seq!(Current, Row).map(|_| CurrentRow),
            Kw::Group.map(|_| Group),
            Kw::Ties.map(|_| Ties),
            seq!(No, Others).map(|_| NoOthers)
        )
    ).parse(ctx)?;

    Ok(exclusion)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("exclude current row", CurrentRow)]
    #[test_case("exclude group", Group)]
    #[test_case("exclude ties", Ties)]
    #[test_case("exclude no others", NoOthers)]
    fn test_window_exclusion_clause(source: &str, expected: WindowExclusion) {
        test_parser!(source, window_exclusion_clause, expected);
    }
}

use pg_ast::WindowExclusion;
use pg_ast::WindowExclusion::CurrentRow;
use pg_ast::WindowExclusion::Group;
use pg_ast::WindowExclusion::NoOthers;
use pg_ast::WindowExclusion::Ties;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Exclude;
use pg_lexer::Keyword::No;
use pg_lexer::Keyword::Others;
use pg_lexer::Keyword::Row;
use pg_parser_core::scan;
