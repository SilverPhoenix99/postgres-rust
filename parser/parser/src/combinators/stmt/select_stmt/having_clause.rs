fn having_clause(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
         HAVING a_expr
    */

    let (_, expr) = seq!(Having, a_expr).parse(ctx)?;
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::BooleanConst;

    #[test]
    fn test_having_clause() {
        test_parser!(
            source = "having true",
            parser = having_clause,
            expected = BooleanConst(true)
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Having;
use pg_parser_core::scan;
