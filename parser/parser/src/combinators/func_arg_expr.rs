pub(super) fn func_arg_list() -> impl Combinator<Output = Vec<FuncArgExpr>> {

    /*
        func_arg_expr ( COMMA func_arg_expr )*
    */

    many!(sep = Comma, func_arg_expr())
}

pub(super) fn func_arg_expr() -> impl Combinator<Output = FuncArgExpr> {

    /*
        type_function_name COLON_EQUALS a_expr
      | type_function_name EQUALS_GREATER a_expr
      | a_expr
    */

    parser(|stream| {

        match stream.peek2_option() {
            Some((first, Operator(ColonEquals | EqualsGreater))) if is_type_function_name(first) => {
                let name = type_function_name.parse(stream)?;
                or(ColonEquals, EqualsGreater).parse(stream)?;
                let value = a_expr().parse(stream)?;
                let arg = NamedValue { name, value };
                Ok(arg)
            },
            _ => {
                let value = a_expr().parse(stream)?;
                let arg = Unnamed(value);
                Ok(arg)
            },
        }
    })
}

fn is_type_function_name(tok: &TokenValue) -> bool {

    match tok {
        Identifier(_) => true,
        Keyword(kw) => matches!(kw.category(), Unreserved | TypeFuncName),
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("1", Unnamed(IntegerConst(1)))]
    #[test_case("foo := 2", NamedValue { name: "foo".into(), value: IntegerConst(2) })]
    #[test_case("bar => 3", NamedValue { name: "bar".into(), value: IntegerConst(3) })]
    fn test_func_arg_expr(source: &str, expected: FuncArgExpr) {
        test_parser!(source, func_arg_expr(), expected);
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::many;
use crate::combinators::foundation::or;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::type_function_name;
use crate::stream::TokenValue;
use crate::stream::TokenValue::Identifier;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::FuncArgExpr;
use pg_ast::FuncArgExpr::NamedValue;
use pg_ast::FuncArgExpr::Unnamed;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_lexer::OperatorKind::ColonEquals;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::EqualsGreater;
