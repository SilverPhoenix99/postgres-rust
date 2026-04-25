pub(super) fn expr_primary_paren(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          '(' SelectStmt ')' ( indirection_el )*
        | '(' a_expr ')' ( indirection_el )*      // 1 element
        | '(' a_expr ',' expr_list ')'            // 2+ elements
    */

    let expr = paren!(expr_or_select).parse(ctx)?;
    if let ExprNode::Row(_) = &expr {
        return Ok(expr)
    }

    let Some(ind) = indirection(ctx).optional()? else {
        return Ok(expr)
    };

    let expr = IndirectionExpr::new(expr, ind);
    Ok(expr.into())
}

fn expr_or_select(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
          SelectStmt
        | expr_list
    */

    if is_select_stmt(ctx) {
        return select_stmt(ctx).map(From::from)
    }

    let mut exprs = expr_list(ctx)?;

    let expr = if exprs.len() == 1 {
        exprs.pop()
            .expect("already checked the length")
    }
    else {
        RowExpr::implicit(exprs).into()
    };

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::Indirection::Property;
    use test_case::test_matrix;

    #[test_matrix("(select 1)" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    #[test_matrix("(select 1).foo" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    #[test_matrix("(1)" => Ok(IntegerConst(1)))]
    #[test_matrix("(1).foo" => Ok(
        IndirectionExpr::new(
            IntegerConst(1),
            vec![Property("foo".into())]
        ).into()
    ))]
    #[test_matrix("(1, 2)" => Ok(
        RowExpr::implicit(vec![
            IntegerConst(1),
            IntegerConst(2)
        ]).into()
    ))]
    fn test_expr_primary_paren(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, expr_primary_paren)
    }

    #[test_matrix("select 1" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    #[test_matrix("1" => Ok(IntegerConst(1)))]
    #[test_matrix("1, 2" => Ok(
        RowExpr::implicit(vec![
            IntegerConst(1),
            IntegerConst(2)
        ]).into()
    ))]
    fn test_expr_or_select(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, expr_or_select)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::indirection;
use crate::combinators::expr_list;
use crate::combinators::stmt::is_select_stmt;
use crate::combinators::stmt::select_stmt;
use crate::context::ParserContext;
use crate::paren;
use pg_ast::ExprNode;
use pg_ast::IndirectionExpr;
use pg_ast::RowExpr;
use pg_parser_core::scan;
use pg_parser_core::Optional;
