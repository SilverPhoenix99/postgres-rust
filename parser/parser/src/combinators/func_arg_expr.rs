pub(super) fn func_arg_list(stream: &mut TokenStream<'_>) -> scan::Result<Vec<Located<NamedValue>>> {

    /*
        func_arg_expr ( COMMA func_arg_expr )*
    */

    many!(sep = Comma, func_arg_expr).parse(stream)
}

pub(super) fn func_arg_expr(stream: &mut TokenStream<'_>) -> scan::Result<Located<NamedValue>> {

    /*
        type_function_name COLON_EQUALS a_expr
      | type_function_name EQUALS_GREATER a_expr
      | a_expr
    */

    match stream.peek2() {
        Ok((first, Operator(ColonEquals | EqualsGreater))) if is_type_function_name(first) => {

            let Located((name, _, value), loc) = located!(seq!(
                type_function_name,
                alt!(ColonEquals, EqualsGreater),
                a_expr
            )).parse(stream)?;

            let arg = NamedValue::new(Some(name), value);
            Ok(Located(arg, loc))
        },
        _ => {
            let Located(value, loc) = located!(a_expr).parse(stream)?;
            let arg = NamedValue::unnamed(value);
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
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("1" => Ok(NamedValue::unnamed(IntegerConst(1))))]
    #[test_case("foo := 2" => Ok(NamedValue::new(Some("foo".into()), IntegerConst(2))))]
    #[test_case("bar => 3" => Ok(NamedValue::new(Some("bar".into()), IntegerConst(3))))]
    fn test_func_arg_expr(source: &str) -> scan::Result<NamedValue> {
        test_parser!(
            source,
            func_arg_expr.map(|Located(arg, _)| arg)
        )
    }
}

use crate::combinators::expr::a_expr;
use pg_ast::NamedValue;
use pg_basics::Located;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_lexer::OperatorKind::ColonEquals;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::EqualsGreater;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue;
use pg_parser_core::stream::TokenValue::Identifier;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
use pg_sink_combinators::type_function_name;
