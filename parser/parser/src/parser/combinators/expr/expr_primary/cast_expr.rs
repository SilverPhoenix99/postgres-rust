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
    use crate::parser::ast_node::ExprNode;
    use crate::parser::ast_node::TypeName::Varchar;
    use crate::parser::tests::test_parser;

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

use crate::parser::ast_node::TypecastExpr;
use crate::parser::combinators::between_paren;
use crate::parser::combinators::expr::a_expr;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::typename;
use postgres_parser_lexer::Keyword::As;
use postgres_parser_lexer::Keyword::Cast;
