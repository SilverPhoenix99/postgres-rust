pub(super) fn func_arg_list(ctx: &mut ParserContext) -> scan::Result<Vec<Located<NamedValue>>> {

    /*
        func_arg_expr ( COMMA func_arg_expr )*
    */

    many!(sep = Comma, func_arg_expr).parse(ctx)
}

pub(super) fn func_arg_expr(ctx: &mut ParserContext) -> scan::Result<Located<NamedValue>> {

    /*
        type_function_name COLON_EQUALS a_expr
      | type_function_name EQUALS_GREATER a_expr
      | a_expr
    */

    match ctx.stream_mut().peek_n::<2>() {
        Ok([first, Operator(ColonEquals | EqualsGreater)]) if is_type_function_name(first) => {

            let Located((name, _, value), loc) = located!(seq!(
                type_function_name,
                alt!(ColonEquals, EqualsGreater),
                a_expr
            )).parse(ctx)?;

            let arg = NamedValue::new(value).with_name(name);
            Ok(Located(arg, loc))
        },
        _ => {
            let Located(value, loc) = located!(a_expr).parse(ctx)?;
            let arg = NamedValue::new(value);
            Ok(Located(arg, loc))
        },
    }
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
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("1" => Ok(
        NamedValue::new(IntegerConst(1))
    ))]
    #[test_matrix("foo := 2" => Ok(
        NamedValue::new(IntegerConst(2))
            .with_name("foo")
    ))]
    #[test_matrix("bar => 3" => Ok(
        NamedValue::new(IntegerConst(3))
            .with_name("bar")
    ))]
    fn test_func_arg_expr(source: &str) -> scan::Result<NamedValue> {
        test_parser!(
            source,
            func_arg_expr.map(|Located(arg, _)| arg)
        )
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::combinators::type_function_name;
use crate::located;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::NamedValue;
use pg_basics::Located;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_lexer::OperatorKind::ColonEquals;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::EqualsGreater;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue;
use pg_parser_core::stream::TokenValue::Identifier;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
