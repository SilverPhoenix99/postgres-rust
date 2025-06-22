pub(super) fn filter_clause() -> impl Combinator<Output = Option<ExprNode>> {
    
    /*
        FILTER '(' WHERE a_expr ')'
    */
    
    Filter
        .and_right(between_paren(
            Where.and_right(a_expr())
        ))
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_filter_clause() {
        test_parser!(
            source = "filter (where true)",
            parser = filter_clause(),
            expected = Some(ExprNode::BooleanConst(true))
        )
    }
}

use crate::combinators::between_paren;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::Combinator;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Filter;
use pg_lexer::Keyword::Where;
