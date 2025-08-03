pub(super) fn json_exists_expr(stream: &mut TokenStream) -> scan::Result<JsonExistsExpr> {

    /*
        JSON_EXISTS '(' json_exists_args ')'
    */

    if ! matches!(stream.peek2(), Ok((K(JsonExists), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    skip_prefix(1, paren(json_exists_args))
        .parse(stream)
}

fn json_exists_args(stream: &mut TokenStream) -> scan::Result<JsonExistsExpr> {

    /*
        json_value_expr
        ','
        a_expr
        ( json_passing_clause )?
        ( json_on_error_clause )?
    */

    let (ctx, _, path_spec, passing, on_error) = seq!(
        json_value_expr,
        Comma,
        a_expr,
        json_passing_clause.optional(),
        json_on_error_clause.optional(),
    ).parse(stream)?;

    let mut expr = JsonExistsExpr::new(ctx, path_spec);
    expr.set_passing(passing)
        .set_on_error(on_error);

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::JsonBehavior,
        pg_ast::JsonValueExpr,
        scan::Error::NoMatch,
    };

    #[test_case("json_exists('{}', 'foo')" => Ok(JsonExistsExpr::new(
        JsonValueExpr::from(StringConst("{}".into())),
        StringConst("foo".into())
    )))]
    #[test_case("json_exists('{}', 'foo' passing 1 as a null on error)" => Ok(
        JsonExistsExpr::new(
            JsonValueExpr::from(StringConst("{}".into())),
            StringConst("foo".into())
        )
        .with_passing(vec![
            ("a".into(), JsonValueExpr::from(IntegerConst(1)))
        ])
        .with_on_error(JsonBehavior::Null)
    ))]
    #[test_case("json_exists" => matches Err(NoMatch(_)))]
    #[test_case("json_exists 1" => matches Err(NoMatch(_)))]
    fn test_json_exists_expr(source: &str) -> scan::Result<JsonExistsExpr> {
        test_parser!(source, json_exists_expr)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_on_error_clause;
use crate::combinators::json_passing_clause;
use crate::combinators::json_value_expr;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::JsonExistsExpr;
use pg_lexer::Keyword::JsonExists;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
