pub(super) fn join_type(ctx: &mut ParserContext) -> scan::Result<JoinKind> {

    /*
          INNER
        | LEFT ( OUTER )?
        | FULL ( OUTER )?
        | RIGHT ( OUTER )?
    */

    alt!(
        Kw::Inner.map(|_| Inner(Some(Default::default()))),
        seq!(
            alt!(
                Kw::Left.map(|_| Left(Default::default())),
                Kw::Full.map(|_| Full(Default::default())),
                Kw::Right.map(|_| Right(Default::default()))
            ),
            Outer.optional()
        ).map(|(kind, _)| kind),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("inner" => Ok(Inner(Some(Default::default()))))]
    #[test_case("left" => Ok(Left(Default::default())))]
    #[test_case("left outer" => Ok(Left(Default::default())))]
    #[test_case("full" => Ok(Full(Default::default())))]
    #[test_case("full outer" => Ok(Full(Default::default())))]
    #[test_case("right" => Ok(Right(Default::default())))]
    #[test_case("right outer" => Ok(Right(Default::default())))]
    fn test_join_type(source: &str) -> scan::Result<JoinKind> {
        test_parser!(source, join_type)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::JoinKind;
use pg_ast::JoinKind::Full;
use pg_ast::JoinKind::Inner;
use pg_ast::JoinKind::Left;
use pg_ast::JoinKind::Right;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Outer;
use pg_parser_core::scan;
