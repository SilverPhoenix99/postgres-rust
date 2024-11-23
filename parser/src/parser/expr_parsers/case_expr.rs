pub(super) fn case_expr() -> impl Combinator<Output = CaseExpr> {

    /*
        CASE ( a_expr )?
            ( WHEN a_expr THEN a_expr )+
            ( ELSE a_expr )?
        END
    */

    sequence!(
        keyword(Case).skip(),
        a_expr().optional(),
        many(when_clause()).required(),
        else_clause()
    ).map(|(_, target, when_clauses, default)|
        CaseExpr::new(target, when_clauses, default)
    )
}

fn when_clause() -> impl Combinator<Output = CaseWhen> {
    sequence!(
        keyword(When).skip(),
        a_expr(),
        keyword(Then).skip(),
        a_expr()
    ).map(|(_, condition, _, body)|
        CaseWhen::new(condition, body)
    )
}

fn else_clause() -> impl Combinator<Output = Option<ExprNode>> {
    keyword(Else)
        .and_right(a_expr())
        .optional()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_when() {
        // TODO
    }
}

use crate::lexer::Keyword::Case;
use crate::lexer::Keyword::Else;
use crate::lexer::Keyword::Then;
use crate::lexer::Keyword::When;
use crate::parser::ast_node::CaseExpr;
use crate::parser::ast_node::CaseWhen;
use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::keyword;
use crate::parser::combinators::many;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::expr_parsers::a_expr;
