pub(super) fn opt_asc_desc(stream: &mut TokenStream) -> scan::Result<Option<SortDirection>> {

    /*
          ASC
        | DESC
        | // empty
    */

    choice!(parsed stream =>
        Asc.map(|_| Ascending),
        Desc.map(|_| Descending),
    )
    .optional()
    .map_err(From::from)
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
        test_parser!(source, opt_asc_desc, expected)
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::SortDirection;
use pg_ast::SortDirection::Ascending;
use pg_ast::SortDirection::Descending;
use pg_lexer::Keyword::Asc;
use pg_lexer::Keyword::Desc;
