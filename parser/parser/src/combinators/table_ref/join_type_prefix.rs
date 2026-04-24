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
    use pg_ast::JoinQual;
    use test_case::test_matrix;

    #[test_matrix("cross" => Ok(
        JoinKind::Inner(None)
    ))]
    #[test_matrix("natural" => Ok(
        JoinKind::Inner(Some(JoinQual::Natural))
    ))]
    #[test_matrix("natural inner" => Ok(
        JoinKind::Inner(Some(JoinQual::Natural))
    ))]
    #[test_matrix("natural full" => Ok(
        JoinKind::Full(JoinQual::Natural)
    ))]
    #[test_matrix("natural left outer" => Ok(
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
