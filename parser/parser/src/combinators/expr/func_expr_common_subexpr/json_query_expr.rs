pub(super) fn json_query_expr(stream: &mut TokenStream) -> scan::Result<JsonQueryExpr> {

    /*
        JSON_QUERY '(' json_query_args ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(json_query_args))
        .parse(stream)?;

    Ok(expr)
}

fn json_query_args(stream: &mut TokenStream) -> scan::Result<JsonQueryExpr> {

    /*
        json_value_expr
        ','
        a_expr
        ( json_passing_clause )?
        ( json_returning_clause )?
        ( json_wrapper_behavior )?
        ( json_quotes_clause )?
        ( json_behavior_clause )?
    */

    let (ctx, _, path_spec, passing, output, wrapper, quotes, behavior) = seq!(
        json_value_expr,
        Comma,
        a_expr,
        json_passing_clause.optional(),
        json_returning_clause.optional(),
        json_wrapper_behavior.optional(),
        json_quotes_clause.optional(),
        json_behavior_clause.optional(),
    ).parse(stream)?;

    let mut expr = JsonQueryExpr::new(ctx, path_spec);
    expr.set_passing(passing)
        .set_output(output)
        .set_wrapper(wrapper)
        .set_quotes(quotes)
        .set_behavior(behavior);

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
        pg_ast::JsonBehaviorClause,
        pg_ast::JsonQuotes,
        pg_ast::JsonValueExpr,
        pg_ast::JsonWrapperBehavior,
        pg_ast::TypeName::Int4,
    };

    #[test_case("json_query('{}', 'foo')" => Ok(
        JsonQueryExpr::new(
            JsonValueExpr::from(StringConst("{}".into())),
            StringConst("foo".into())
        )
    ))]
    #[test_case("json_query('{}', 'foo' passing 1 as a returning int with wrapper keep quotes error on empty)" => Ok(
        JsonQueryExpr::new(
            JsonValueExpr::from(StringConst("{}".into())),
            StringConst("foo".into())
        )
        .with_passing(vec![
            ("a".into(), JsonValueExpr::from(IntegerConst(1)))
        ])
        .with_output(Int4.into())
        .with_wrapper(JsonWrapperBehavior::Unconditional)
        .with_quotes(JsonQuotes::Keep)
        .with_behavior(
            JsonBehaviorClause::new()
                .with_on_empty(JsonBehavior::Error)
        )
    ))]
    fn test_json_query_expr(source: &str) -> scan::Result<JsonQueryExpr> {
        test_parser!(source, json_query_expr)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_behavior_clause;
use crate::combinators::json_passing_clause;
use crate::combinators::json_quotes_clause;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use crate::combinators::json_wrapper_behavior;
use crate::stream::TokenStream;
use pg_ast::JsonQueryExpr;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
