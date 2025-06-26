pub(super) fn cast_expr() -> impl Combinator<Output = TypecastExpr> {
    /*
        CAST '(' a_expr AS Typename ')'
    */

    Cast.and_right(parser(|stream| between!(paren : stream =>
        (
            a_expr(),
            As,
            typename
        )
            .map(|(arg, _, type_name)|
                TypecastExpr::new(arg, type_name)
            )
            .parse(stream)
    )))
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
            parser = cast_expr(),
            expected = TypecastExpr::new(
                ExprNode::IntegerConst(1),
                Varchar { max_length: None }
            )
        )
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::between;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use pg_ast::TypecastExpr;
use pg_lexer::Keyword::As;
use pg_lexer::Keyword::Cast;
