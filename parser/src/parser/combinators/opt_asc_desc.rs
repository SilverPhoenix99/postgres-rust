pub(super) fn opt_asc_desc() -> impl Combinator<Output = SortDirection> {

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
    .map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("asc", Ascending)]
    #[test_case("desc", Descending)]
    #[test_case("foo", SortDirection::Default)]
    #[test_case("", SortDirection::Default)]
    fn test_opt_asc_desc(source: &str, expected: SortDirection) {
        test_parser!(source, opt_asc_desc(), expected)
    }
}

use crate::lexer::Keyword::Asc;
use crate::lexer::Keyword::Desc;
use crate::parser::ast_node::SortDirection;
use crate::parser::ast_node::SortDirection::Ascending;
use crate::parser::ast_node::SortDirection::Descending;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
