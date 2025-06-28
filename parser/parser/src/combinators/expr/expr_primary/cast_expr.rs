pub(super) fn cast_expr(stream: &mut TokenStream) -> scan::Result<TypecastExpr> {
    /*
        CAST '(' a_expr AS Typename ')'
    */

    let (_, (arg, _, type_name)) = seq!(=>
        Cast.parse(stream),
        between!(paren : stream =>
            seq!(stream => a_expr, As, typename)
        )
    )?;

    let expr = TypecastExpr::new(arg, type_name);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
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

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::between;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Cast;
