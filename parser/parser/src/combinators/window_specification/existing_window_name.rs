/// Alias: `opt_existing_window_name`
pub(super) fn existing_window_name(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        col_id
    */

    let tokens = ctx.stream_mut().peek_n::<2>()?;

    // These 2 rules need to be checked first, due to conflicts with Unreserved keywords.
    if {
        matches!(tokens,
            [Kw(Partition), Kw(By)]
            | [Kw(RangeKw | Rows | Groups), Kw(Unbounded | Current | Between)]
        )
    } {
        return no_match(ctx)
    }

    col_id(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::core::Combinator;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("partition by" => Ok(None))]
    #[test_matrix("partition partition" => Ok(Some("partition".into())))]
    #[test_matrix("partition range" => Ok(Some("partition".into())))]
    #[test_matrix("partition rows" => Ok(Some("partition".into())))]
    #[test_matrix("partition groups" => Ok(Some("partition".into())))]
    #[test_matrix("range between" => Ok(None))]
    #[test_matrix("range unbounded" => Ok(None))]
    #[test_matrix("range current" => Ok(None))]
    #[test_matrix("range partition" => Ok(Some("range".into())))]
    #[test_matrix("range range" => Ok(Some("range".into())))]
    #[test_matrix("range rows" => Ok(Some("range".into())))]
    #[test_matrix("range groups" => Ok(Some("range".into())))]
    #[test_matrix("rows between" => Ok(None))]
    #[test_matrix("rows unbounded" => Ok(None))]
    #[test_matrix("rows current" => Ok(None))]
    #[test_matrix("rows partition" => Ok(Some("rows".into())))]
    #[test_matrix("rows range" => Ok(Some("rows".into())))]
    #[test_matrix("rows rows" => Ok(Some("rows".into())))]
    #[test_matrix("rows groups" => Ok(Some("rows".into())))]
    #[test_matrix("groups between" => Ok(None))]
    #[test_matrix("groups unbounded" => Ok(None))]
    #[test_matrix("groups current" => Ok(None))]
    #[test_matrix("groups partition" => Ok(Some("groups".into())))]
    #[test_matrix("groups range" => Ok(Some("groups".into())))]
    #[test_matrix("groups rows" => Ok(Some("groups".into())))]
    #[test_matrix("groups groups" => Ok(Some("groups".into())))]
    #[test_matrix("something else" => Ok(Some("something".into())))]
    #[test_matrix("" => Ok(None))]
    fn test_existing_window_name(source: &str) -> scan::Result<Option<Str>> {
        test_parser!(source, existing_window_name.optional())
    }
}

use crate::combinators::col_id;
use crate::no_match;
use crate::ParserContext;
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
use pg_parser_core::stream::TokenValue::Keyword as Kw;
