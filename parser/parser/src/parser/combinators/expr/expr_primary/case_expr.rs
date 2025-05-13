pub(super) fn case_expr() -> impl Combinator<Output = CaseExpr> {

    /*
        CASE ( a_expr )?
            ( WHEN a_expr THEN a_expr )+
            ( ELSE a_expr )?
        END
    */

    sequence!(
        Case.skip(),
        a_expr().optional(),
        many(when_clause()),
        else_clause()
    ).map(|(_, target, when_clauses, default)|
        CaseExpr::new(target, when_clauses, default)
    )
}

fn when_clause() -> impl Combinator<Output = CaseWhen> {
    and(
        When.and_right(a_expr()),
        Then.and_right(a_expr()),
    ).map(|(condition, body)|
        CaseWhen::new(condition, body)
    )
}

fn else_clause() -> impl Combinator<Output = Option<ExprNode>> {
    Else
        .and_right(a_expr())
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_case_value_when_else() {
        test_parser!(
            source = "
                CASE 'foo'
                    WHEN 1 THEN 2
                    ELSE 3
                END
            ",
            parser = case_expr(),
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
            parser = case_expr(),
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

use crate::parser::combinators::expr::a_expr;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_ast::CaseExpr;
use postgres_parser_ast::CaseWhen;
use postgres_parser_ast::ExprNode;
use postgres_parser_lexer::Keyword::Case;
use postgres_parser_lexer::Keyword::Else;
use postgres_parser_lexer::Keyword::Then;
use postgres_parser_lexer::Keyword::When;
