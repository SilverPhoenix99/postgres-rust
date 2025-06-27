pub(super) fn opt_existing_window_name(stream: &mut TokenStream<'_>) -> scan::Result<Option<Str>> {

    /*
        ( col_id )?
    */

    let Some((first, second)) = stream.peek2_option() else {
        return Ok(None)
    };

    match (first, second) {
        // These 2 rules need to come first, due to conflicts with Unreserved keywords.
        (Kw(Partition), Kw(By)) => Ok(None),
        (Kw(RangeKw | Rows | Groups), Kw(Unbounded | Current | Between)) => Ok(None),

        // ColId:
        (Identifier(_), _) => {
            let name = identifier(stream)?;
            Ok(Some(name.into()))
        },
        (Kw(kw), _) if matches!(kw.category(), Unreserved | ColumnName) => {
            let name = any_keyword().parse(stream)?;
            Ok(Some(name.into()))
        },

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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
    fn test_opt_existing_window_name(source: &str, expected: Option<Str>) {
        test_parser!(source, opt_existing_window_name, expected);
    }
}

use crate::combinators::foundation::any_keyword;
use crate::combinators::foundation::identifier;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Identifier;
use crate::stream::TokenValue::Keyword as Kw;
use pg_basics::Str;
use pg_lexer::Keyword::Between;
use pg_lexer::Keyword::By;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Groups;
use pg_lexer::Keyword::Partition;
use pg_lexer::Keyword::RangeKw;
use pg_lexer::Keyword::Rows;
use pg_lexer::Keyword::Unbounded;
use pg_lexer::KeywordCategory::ColumnName;
use pg_lexer::KeywordCategory::Unreserved;
