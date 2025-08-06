/// Alias: `opt_existing_window_name`
pub(super) fn existing_window_name(stream: &mut TokenStream<'_>) -> scan::Result<Str> {

    /*
        col_id
    */

    let tokens = stream.peek2()?;

    // These 2 rules need to be checked first, due to conflicts with Unreserved keywords.
    if {
        matches!(tokens,
            (Kw(Partition), Kw(By))
            | (Kw(RangeKw | Rows | Groups), Kw(Unbounded | Current | Between))
        )
    } {
        return no_match(stream)
    }

    col_id(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_combinators::Combinator;
    use test_case::test_case;

    #[test_case("partition by", None)]
    #[test_case("partition partition", Some("partition".into()))]
    #[test_case("partition range", Some("partition".into()))]
    #[test_case("partition rows", Some("partition".into()))]
    #[test_case("partition groups", Some("partition".into()))]
    #[test_case("range between", None)]
    #[test_case("range unbounded", None)]
    #[test_case("range current", None)]
    #[test_case("range partition", Some("range".into()))]
    #[test_case("range range", Some("range".into()))]
    #[test_case("range rows", Some("range".into()))]
    #[test_case("range groups", Some("range".into()))]
    #[test_case("rows between", None)]
    #[test_case("rows unbounded", None)]
    #[test_case("rows current", None)]
    #[test_case("rows partition", Some("rows".into()))]
    #[test_case("rows range", Some("rows".into()))]
    #[test_case("rows rows", Some("rows".into()))]
    #[test_case("rows groups", Some("rows".into()))]
    #[test_case("groups between", None)]
    #[test_case("groups unbounded", None)]
    #[test_case("groups current", None)]
    #[test_case("groups partition", Some("groups".into()))]
    #[test_case("groups range", Some("groups".into()))]
    #[test_case("groups rows", Some("groups".into()))]
    #[test_case("groups groups", Some("groups".into()))]
    #[test_case("something else", Some("something".into()))]
    #[test_case("", None)]
    fn test_existing_window_name(source: &str, expected: Option<Str>) {
        test_parser!(source, existing_window_name.optional(), expected);
    }
}

use crate::combinators::col_id;
use crate::no_match;
use pg_basics::Str;
use pg_lexer::Keyword::Between;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Groups;
use pg_lexer::Keyword::Partition;
use pg_lexer::Keyword::RangeKw;
use pg_lexer::Keyword::Rows;
use pg_lexer::Keyword::Unbounded;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Keyword as Kw;
