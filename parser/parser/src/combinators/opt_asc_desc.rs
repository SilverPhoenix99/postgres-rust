pub(super) fn opt_asc_desc() -> impl Combinator<Output = Option<SortDirection>> {

    /*
          ASC
        | DESC
        | // empty
    */

    or(
        Asc.map(|_| Ascending),
        Desc.map(|_| Descending),
    )
    .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("asc", Some(Ascending))]
    #[test_case("desc", Some(Descending))]
    #[test_case("foo", None)]
    #[test_case("", None)]
    fn test_opt_asc_desc(source: &str, expected: Option<SortDirection>) {
        test_parser!(source, opt_asc_desc(), expected)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::SortDirection;
use postgres_parser_ast::SortDirection::Ascending;
use postgres_parser_ast::SortDirection::Descending;
use postgres_parser_lexer::Keyword::Asc;
use postgres_parser_lexer::Keyword::Desc;
