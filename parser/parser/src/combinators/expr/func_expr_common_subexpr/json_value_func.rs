pub(super) fn json_value_func(stream: &mut TokenStream) -> scan::Result<JsonValueFunc> {

    /*
        JSON_VALUE '(' json_value_args ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(json_value_args))
        .parse(stream)?;

    Ok(expr)
}

fn json_value_args(stream: &mut TokenStream) -> scan::Result<JsonValueFunc> {

    /*
        json_value_expr
        ','
        a_expr
        ( json_passing_clause )?
        ( json_returning_clause )?
        ( json_behavior_clause )?
    */

    let (value, _, path_spec, passing, output, behavior) = seq!(
        json_value_expr,
        Comma,
        a_expr,
        json_passing_clause.optional(),
        json_returning_clause.optional(),
        json_behavior_clause.optional(),
    ).parse(stream)?;

    let mut func = JsonValueFunc::new(value, path_spec);
    func.set_passing(passing)
        .set_output(output)
        .set_behavior(behavior.unwrap_or_default());

    Ok(func)
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
        pg_ast::JsonBehaviorClause,
        pg_ast::JsonOutput,
        pg_ast::JsonValueExpr,
        pg_ast::TypeName::Int4,
    };

    #[test_case("json_value('{}', 'foo')" => Ok(JsonValueFunc::new(
        JsonValueExpr::from(StringConst("{}".into())),
        StringConst("foo".into())
    )))]
    #[test_case("json_value('{}', 'foo' passing 1 as a returning int null on error)" => Ok(
        JsonValueFunc::new(
            JsonValueExpr::from(StringConst("{}".into())),
            StringConst("foo".into())
        )
        .with_passing(vec![
            ("a".into(), JsonValueExpr::from(IntegerConst(1)))
        ])
        .with_output(JsonOutput::from(Int4))
        .with_behavior(
            JsonBehaviorClause::new()
                .with_on_error(JsonBehavior::Null)
        )
    ))]
    fn test_json_value_func(source: &str) -> scan::Result<JsonValueFunc> {
        test_parser!(source, json_value_func)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::json_behavior_clause;
use crate::combinators::json_passing_clause;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use pg_ast::JsonValueFunc;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
