/// Alias: `opt_asc_desc`
pub(super) fn asc_desc(stream: &mut TokenStream) -> scan::Result<SortDirection> {

    /*
          ASC
        | DESC
    */

    alt!(
        Asc.map(|_| Ascending),
        Desc.map(|_| Descending),
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("asc", Ascending)]
    #[test_case("desc", Descending)]
    fn test_asc_desc(source: &str, expected: SortDirection) {
        test_parser!(source, asc_desc, expected)
    }
}

use crate::combinators::foundation::alt;
use pg_ast::SortDirection;
use pg_ast::SortDirection::Ascending;
use pg_ast::SortDirection::Descending;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Asc;
use pg_lexer::Keyword::Desc;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
