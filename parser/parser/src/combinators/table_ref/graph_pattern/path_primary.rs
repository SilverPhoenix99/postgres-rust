pub(super) fn path_primary(ctx: &mut ParserContext) -> scan::Result<GraphElementPatternKind> {

    /*
        | right_arrow                               => EdgePatternRight
        | left_arrow                                => EdgePatternLeft
        | left_arrow '[' path_primary_expr ']' '-'  => EdgePatternLeft
        | '-'                                       => EdgePatternAny
        | '-' '[' path_primary_expr ']' '-'         => EdgePatternAny
        | '-' '[' path_primary_expr ']' right_arrow => EdgePatternRight
        | TODO: '(' path_term ( where_clause )? ')'       => ParenExpr
        | TODO: '(' path_primary_expr           ')'       => VertexPattern
    */

    let kind: GraphElementPatternKind = alt!(
        right_arrow_pattern,
        left_arrow_pattern,
        any_pattern
    ).parse(ctx)?;

    Ok(kind)
}

fn right_arrow_pattern(ctx: &mut ParserContext) -> scan::Result<GraphElementPatternKind> {

    /*
        right_arrow
    */

    right_arrow(ctx)?;
    Ok(EdgePatternRight(GraphElementPattern::default()))
}

fn left_arrow_pattern(ctx: &mut ParserContext) -> scan::Result<GraphElementPatternKind> {

    /*
          left_arrow
        | left_arrow '[' path_primary_expr ']' '-'
    */

    let (_, pattern) = seq!(
        left_arrow,
        seq!(brackets!(path_primary_expr), Minus)
            .map(|(pattern, _)| pattern)
            .optional()
    ).parse(ctx)?;

    let pattern = pattern.unwrap_or_default();
    Ok(EdgePatternLeft(pattern))
}

fn any_pattern(ctx: &mut ParserContext) -> scan::Result<GraphElementPatternKind> {

    /*
          '-'
        | '-' '[' path_primary_expr ']' right_arrow
        | '-' '[' path_primary_expr ']' '-'
    */

    let (_, pattern) = seq!(
        Minus,
        seq!(
            brackets!(path_primary_expr),
            alt!(
                right_arrow.map(|_| RightArrow),
                Minus
            )
        )
            .map(|(pattern, op)| {
                if op == RightArrow {
                    EdgePatternRight(pattern)
                }
                else {
                    EdgePatternAny(pattern)
                }
            })
            .optional()
    ).parse(ctx)?;

    let pattern = pattern.unwrap_or_else(||
        EdgePatternAny(GraphElementPattern::default())
    );

    Ok(pattern)
}

fn right_arrow(ctx: &mut ParserContext) -> scan::Result<()> {

    /*
          RIGHT_ARROW
        | '-' '>'
    */

    let stream = ctx.stream_mut();

    if matches!(stream.peek(), Ok(Operator(RightArrow))) {
        stream.skip(1);
    }
    else if matches!(stream.peek2(), Ok((Operator(Minus), Operator(Greater)))) {
        stream.skip(2);
    }
    else {
        return no_match(ctx);
    }

    Ok(())
}

fn left_arrow(ctx: &mut ParserContext) -> scan::Result<()> {

    /*
        '<' '-'
    */

    seq!(Less, Minus).parse(ctx)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("->" => matches Ok(_); "right pattern")]
    #[test_case("<-" => matches Ok(_); "left pattern")]
    #[test_case("-" => matches Ok(_); "any pattern")]
    fn test_path_primary(source: &str) -> scan::Result<GraphElementPatternKind> {
        test_parser!(source, path_primary)
    }

    #[test_case("->" => Ok(EdgePatternRight(
        GraphElementPattern::default()
    )))]
    fn test_right_arrow_pattern(source: &str) -> scan::Result<GraphElementPatternKind> {
        test_parser!(source, right_arrow_pattern)
    }

    #[test_case("<-" => Ok(EdgePatternLeft(
        GraphElementPattern::default()
    )))]
    #[test_case("<-[foo]-" => Ok(EdgePatternLeft(
        GraphElementPattern::default()
            .with_variable("foo")
    )))]
    fn test_left_arrow_pattern(source: &str) -> scan::Result<GraphElementPatternKind> {
        test_parser!(source, left_arrow_pattern)
    }

    #[test_case("-" => Ok(EdgePatternAny(
        GraphElementPattern::default()
    )); "dash")]
    #[test_case("-[foo]-" => Ok(EdgePatternAny(
        GraphElementPattern::default()
            .with_variable("foo")
    )); "double-dash")]
    #[test_case("-[is a]->" => Ok(EdgePatternRight(
        GraphElementPattern::default()
            .with_label_expr(vec!["a".into()])
    )); "dash-right-arrow")]
    fn test_any_pattern(source: &str) -> scan::Result<GraphElementPatternKind> {
        test_parser!(source, any_pattern)
    }

    #[test_case("->" => Ok(()); "right-arrow")]
    #[test_case("- >" => Ok(()); "dash-greater")]
    fn test_right_arrow(source: &str) -> scan::Result<()> {
        test_parser!(source, right_arrow)
    }

    #[test_case("<-" => Ok(()); "left-arrow")]
    #[test_case("< -" => Ok(()); "less-dash")]
    fn test_left_arrow(source: &str) -> scan::Result<()> {
        test_parser!(source, left_arrow)
    }
}

use super::path_primary_expr;
use crate::alt;
use crate::brackets;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::no_match;
use crate::seq;
use pg_ast::GraphElementPattern;
use pg_ast::GraphElementPatternKind;
use pg_ast::GraphElementPatternKind::EdgePatternAny;
use pg_ast::GraphElementPatternKind::EdgePatternLeft;
use pg_ast::GraphElementPatternKind::EdgePatternRight;
use pg_lexer::OperatorKind::Greater;
use pg_lexer::OperatorKind::Less;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::RightArrow;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Operator;
