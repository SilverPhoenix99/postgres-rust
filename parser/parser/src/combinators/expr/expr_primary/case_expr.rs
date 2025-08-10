pub(super) fn case_expr(ctx: &mut ParserContext) -> scan::Result<CaseExpr> {

    /*
        CASE ( a_expr )?
            ( WHEN a_expr THEN a_expr )+
            ( ELSE a_expr )?
        END
    */

    let (_, target, when_clauses, default) = seq!(
        Case,
        a_expr.optional(),
        many!(when_clause),
        else_clause.optional()
    ).parse(ctx)?;

    let expr = CaseExpr::new(target, when_clauses, default);
    Ok(expr)
}

fn when_clause(ctx: &mut ParserContext) -> scan::Result<CaseWhen> {

    let (_, condition, _, body) = seq!(When, a_expr, Then, a_expr).parse(ctx)?;

    let expr = CaseWhen::new(condition, body);
    Ok(expr)
}

fn else_clause(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    let (_, expr) = seq!(Else, a_expr).parse(ctx)?;

    Ok(expr)

}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_case_value_when_else() {
        test_parser!(
            source = "
                CASE 'foo'
                    WHEN 1 THEN 2
                    ELSE 3
                END
            ",
            parser = case_expr,
            expected = CaseExpr::new(
                Some(ExprNode::StringConst("foo".into())),
                vec![
                    CaseWhen::new(
                        ExprNode::IntegerConst(1),
                        ExprNode::IntegerConst(2),
                    ),
                ],
                Some(ExprNode::IntegerConst(3)),
            )
        )
    }

    #[test]
    fn test_case_when() {
        test_parser!(
            source = "
                CASE
                    WHEN 1 THEN 2
                END
            ",
            parser = case_expr,
            expected = CaseExpr::new(
                None,
                vec![
                    CaseWhen::new(
                        ExprNode::IntegerConst(1),
                        ExprNode::IntegerConst(2),
                    ),
                ],
                None
            )
        )
    }
}

use crate::combinators::expr::a_expr;
use pg_ast::CaseExpr;
use pg_ast::CaseWhen;
use pg_ast::ExprNode;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Case;
use pg_lexer::Keyword::Else;
use pg_lexer::Keyword::Then;
use pg_lexer::Keyword::When;
use pg_parser_core::scan;
