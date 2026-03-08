pub(in crate::combinators::expr::expr_primary)
fn filter_clause(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        FILTER '(' WHERE a_expr ')'
    */

    let (_, (_, expr)) = seq!(
        Filter,
        paren!(seq!(Where, a_expr))
    ).parse(ctx)?;

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_filter_clause() {
        test_parser!(
            source = "filter (where true)",
            parser = filter_clause,
            expected = ExprNode::BooleanConst(true)
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Filter;
use pg_lexer::Keyword::Where;
use pg_parser_core::scan;
