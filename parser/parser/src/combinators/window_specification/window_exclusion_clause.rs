/// Alias: `opt_window_exclusion_clause`
pub(super) fn window_exclusion_clause(stream: &mut TokenStream<'_>) -> scan::Result<WindowExclusion> {

    /*
          EXCLUDE CURRENT ROW
        | EXCLUDE GROUP
        | EXCLUDE TIES
        | EXCLUDE NO OTHERS
    */

    let (_, exclusion) = (
        Exclude,
        or((
            (Current, Row).map(|_| CurrentRow),
            Kw::Group.map(|_| Group),
            Kw::Ties.map(|_| Ties),
            (No, Others).map(|_| NoOthers)
        ))
    ).parse(stream)?;

    Ok(exclusion)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("exclude current row", CurrentRow)]
    #[test_case("exclude group", Group)]
    #[test_case("exclude ties", Ties)]
    #[test_case("exclude no others", NoOthers)]
    fn test_window_exclusion_clause(source: &str, expected: WindowExclusion) {
        test_parser!(source, window_exclusion_clause, expected);
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::WindowExclusion;
use pg_ast::WindowExclusion::CurrentRow;
use pg_ast::WindowExclusion::Group;
use pg_ast::WindowExclusion::NoOthers;
use pg_ast::WindowExclusion::Ties;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Exclude;
use pg_lexer::Keyword::No;
use pg_lexer::Keyword::Others;
use pg_lexer::Keyword::Row;
