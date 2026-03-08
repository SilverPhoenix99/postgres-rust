pub(super) fn cast_expr(ctx: &mut ParserContext) -> scan::Result<TypecastExpr> {

    /*
        CAST '(' a_expr AS Typename ')'
    */

    let (_, (arg, _, type_name)) = seq!(
        Cast,
        paren!(seq!(a_expr, As, typename))
    ).parse(ctx)?;

    let expr = TypecastExpr::new(arg, type_name);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode;
    use pg_ast::TypeName::Varchar;

    #[test]
    fn test_cast_expr() {
        test_parser!(
            source = "cast (1 as varchar)",
            parser = cast_expr,
            expected = TypecastExpr::new(
                ExprNode::IntegerConst(1),
                Varchar { max_length: None }
            )
        )
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::combinators::typename;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Cast;
use pg_parser_core::scan;
