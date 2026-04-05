pub(super) fn join_type_prefix(ctx: &mut ParserContext) -> scan::Result<JoinKind> {

    /*
          CROSS
        | NATURAL ( join_type )?
    */

    alt!(
        Cross.map(|_| JoinKind::cross_join()),
        seq!(Natural, join_type.optional())
            .map(|(_, join_type)|
                join_type.unwrap_or_default()
            )
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::JoinQual;
    use test_case::test_case;

    #[test_case("cross" => Ok(
        JoinKind::Inner(None)
    ))]
    #[test_case("natural" => Ok(
        JoinKind::Inner(Some(JoinQual::Natural))
    ))]
    #[test_case("natural inner" => Ok(
        JoinKind::Inner(Some(JoinQual::Natural))
    ))]
    #[test_case("natural full" => Ok(
        JoinKind::Full(JoinQual::Natural)
    ))]
    #[test_case("natural left outer" => Ok(
        JoinKind::Left(JoinQual::Natural)
    ))]
    fn test_join_type_prefix(source: &str) -> scan::Result<JoinKind> {
        test_parser!(source, join_type_prefix)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::table_ref::join_type;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::JoinKind;
use pg_lexer::Keyword::Cross;
use pg_lexer::Keyword::Natural;
use pg_parser_core::scan;
