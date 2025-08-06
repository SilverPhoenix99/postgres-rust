pub(super) fn json_exists_expr(stream: &mut TokenStream) -> scan::Result<JsonExistsExpr> {

    /*
        JSON_EXISTS '(' json_exists_args ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(json_exists_args))
        .parse(stream)?;

    Ok(expr)
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
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::JsonBehavior,
        pg_ast::JsonValueExpr,
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
    fn test_json_exists_expr(source: &str) -> scan::Result<JsonExistsExpr> {
        test_parser!(source, json_exists_expr)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::json_on_error_clause;
use crate::combinators::json_passing_clause;
use crate::combinators::json_value_expr;
use pg_ast::JsonExistsExpr;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
