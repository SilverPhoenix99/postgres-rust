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

            let ((name, _, value), loc) = located!(seq!(
                type_function_name,
                alt!(ColonEquals, EqualsGreater),
                a_expr
            )).parse(stream)?;

            let arg = NamedValue::new(Some(name), value);
            Ok((arg, loc))
        },
        _ => {
            let (value, loc) = located!(a_expr).parse(stream)?;
            let arg = NamedValue::unnamed(value);
            Ok((arg, loc))
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
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("1" => Ok(NamedValue::unnamed(IntegerConst(1))))]
    #[test_case("foo := 2" => Ok(NamedValue::new(Some("foo".into()), IntegerConst(2))))]
    #[test_case("bar => 3" => Ok(NamedValue::new(Some("bar".into()), IntegerConst(3))))]
    fn test_func_arg_expr(source: &str) -> scan::Result<NamedValue> {
        test_parser!(
            source,
            func_arg_expr.map(|(arg, _)| arg)
        )
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::type_function_name;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use crate::stream::TokenValue::Identifier;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::NamedValue;
use pg_basics::Located;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_lexer::OperatorKind::ColonEquals;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::EqualsGreater;
use pg_parser_core::scan;
