pub(super) fn opt_window_exclusion_clause(stream: &mut TokenStream<'_>) -> scan::Result<Option<WindowExclusion>> {

    /*
          EXCLUDE CURRENT ROW
        | EXCLUDE GROUP
        | EXCLUDE TIES
        | EXCLUDE NO OTHERS
    */

    let exclusion = (
        Exclude,
        or((
            (Current, Row).map(|_| Some(CurrentRow)),
            Kw::Group.map(|_| Some(Group)),
            Kw::Ties.map(|_| Some(Ties)),
            (No, Others).map(|_| None)
        ))
    ).parse(stream);

    let exclusion = exclusion.optional()?
        .and_then(|(_, exclusion)| exclusion);

    Ok(exclusion)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("exclude current row", Some(WindowExclusion::CurrentRow))]
    #[test_case("exclude group", Some(WindowExclusion::Group))]
    #[test_case("exclude ties", Some(WindowExclusion::Ties))]
    #[test_case("exclude no others", None)]
    #[test_case("something else", None)]
    #[test_case("", None)]
    fn test_opt_window_exclusion_clause(source: &str, expected: Option<WindowExclusion>) {
        test_parser!(source, opt_window_exclusion_clause, expected);
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::WindowExclusion;
use pg_ast::WindowExclusion::CurrentRow;
use pg_ast::WindowExclusion::Group;
use pg_ast::WindowExclusion::Ties;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Exclude;
use pg_lexer::Keyword::No;
use pg_lexer::Keyword::Others;
use pg_lexer::Keyword::Row;
