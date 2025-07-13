pub(super) fn func_arg_list(stream: &mut TokenStream<'_>) -> scan::Result<Vec<Located<FuncArgExpr>>> {

    /*
        func_arg_expr ( COMMA func_arg_expr )*
    */

    many_sep(Comma, func_arg_expr).parse(stream)
}

pub(super) fn func_arg_expr(stream: &mut TokenStream<'_>) -> scan::Result<Located<FuncArgExpr>> {

    /*
        type_function_name COLON_EQUALS a_expr
      | type_function_name EQUALS_GREATER a_expr
      | a_expr
    */

    match stream.peek2() {
        Ok((first, Operator(ColonEquals | EqualsGreater))) if is_type_function_name(first) => {

            let ((name, _, value), loc) = located((
                type_function_name,
                or((ColonEquals, EqualsGreater)),
                a_expr
            )).parse(stream)?;

            let arg = NamedValue { name, value };
            Ok((arg, loc))
        },
        _ => {
            let (value, loc) = located(a_expr).parse(stream)?;
            let arg = Unnamed(value);
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
    use crate::tests::stream;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("1", Unnamed(IntegerConst(1)))]
    #[test_case("foo := 2", NamedValue { name: "foo".into(), value: IntegerConst(2) })]
    #[test_case("bar => 3", NamedValue { name: "bar".into(), value: IntegerConst(3) })]
    fn test_func_arg_expr(source: &str, expected: FuncArgExpr) {
        let mut stream = stream(source);
        let (actual, _) = func_arg_expr(&mut stream).unwrap();
        assert_eq!(expected, actual)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::located;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::type_function_name;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue;
use crate::stream::TokenValue::Identifier;
use crate::stream::TokenValue::Keyword;
use crate::stream::TokenValue::Operator;
use pg_ast::FuncArgExpr;
use pg_ast::FuncArgExpr::NamedValue;
use pg_ast::FuncArgExpr::Unnamed;
use pg_basics::Located;
use pg_lexer::KeywordCategory::TypeFuncName;
use pg_lexer::KeywordCategory::Unreserved;
use pg_lexer::OperatorKind::ColonEquals;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::EqualsGreater;
