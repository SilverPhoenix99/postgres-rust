pub(super) fn row(ctx: &mut ParserContext) -> scan::Result<Option<Vec<ExprNode>>> {

    alt!(
        explicit_row,
        implicit_row.map(Some)
    ).parse(ctx)
}

pub(super) fn explicit_row(ctx: &mut ParserContext) -> scan::Result<Option<Vec<ExprNode>>> {

    /*
        ROW '(' ( expr_list )? ')'
    */

    if !matches!(ctx.stream_mut().peek_n::<2>(), Ok([Keyword(Kw::Row), Operator(OpenParenthesis)])) {
        return no_match(ctx)
    }

    let (_, col_values) = seq!(skip(1), paren!(expr_list.optional()))
        .parse(ctx)?;

    Ok(col_values)
}

fn implicit_row(ctx: &mut ParserContext) -> scan::Result<Vec<ExprNode>> {

    /*
        '(' a_expr ',' expr_list ')' // 2+ elements
    */

    let (first, _, mut expressions) = paren!(seq!(a_expr, Comma, expr_list)).parse(ctx)?;
    expressions.insert(0, first);

    Ok(expressions)
}

pub(super) fn overlaps_row(ctx: &mut ParserContext) -> scan::Result<Located<Option<Vec<ExprNode>>>> {

    /*
        OVERLAPS row
    */

    if !matches!(ctx.stream_mut().peek_n::<2>(), Ok([Keyword(Overlaps), Keyword(Kw::Row) | Operator(OpenParenthesis)])) {
        return no_match(ctx)
    }

    let (_, row) = seq!(skip(1), located!(row)).parse(ctx)?;

    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use test_case::{test_case, test_matrix};

    #[test_matrix(
        [
            "row()",
            "row(1)",
            "row(2, 'foo')",
            "(3, 'bar')",
        ]
        => matches Ok(_)
    )]
    fn test_row(source: &str) -> scan::Result<Option<Vec<ExprNode>>> {
        test_parser!(source, row)
    }

    #[test_case("row()" => Ok(None))]
    #[test_case("row(1)" => Ok(Some(vec![IntegerConst(1)])))]
    #[test_case("row(1, 'foo')" => Ok(Some(vec![IntegerConst(1), StringConst("foo".into())])))]
    fn test_explicit_row(source: &str) -> scan::Result<Option<Vec<ExprNode>>> {
        test_parser!(source, explicit_row)
    }

    #[test_case("(1, 'foo')" => Ok(vec![IntegerConst(1), StringConst("foo".into())]))]
    fn test_implicit_row(source: &str) -> scan::Result<Vec<ExprNode>> {
        test_parser!(source, implicit_row)
    }

    #[test_matrix(
        [
            "overlaps row()",
            "overlaps row(1)",
            "overlaps row(2, 'foo')",
            "overlaps (3, 'bar')",
        ]
        => matches Ok(_)
    )]
    fn test_overlaps_row(source: &str) -> scan::Result<Located<Option<Vec<ExprNode>>>> {
        test_parser!(source, overlaps_row)
    }
}

use crate::alt;
use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::combinators::expr_list;
use crate::located;
use crate::no_match;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_basics::Located;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Overlaps;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
