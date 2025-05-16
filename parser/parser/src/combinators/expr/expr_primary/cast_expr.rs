pub(super) fn cast_expr() -> impl Combinator<Output = TypecastExpr> {
    /*
        CAST '(' a_expr AS Typename ')'
    */

    Cast
        .and_right(between_paren(
            a_expr()
                .and_then(
                    As.and_right(typename()),
                    TypecastExpr::new
                )
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use postgres_parser_ast::ExprNode;
    use postgres_parser_ast::TypeName::Varchar;

    #[test]
    fn test_cast_expr() {
        test_parser!(
            source = "cast (1 as varchar)",
            parser = cast_expr(),
            expected = TypecastExpr::new(
                ExprNode::IntegerConst(1),
                Varchar { max_length: None }.into()
            )
        )
    }
}

use crate::combinators::between_paren;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::typename;
use postgres_parser_ast::TypecastExpr;
use postgres_parser_lexer::Keyword::As;
use postgres_parser_lexer::Keyword::Cast;
