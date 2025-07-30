pub(super) fn json_value_func(stream: &mut TokenStream) -> scan::Result<JsonValueFunc> {

    /*
        JSON_VALUE '('
            json_value_expr
            ','
            a_expr
            ( json_passing_clause )?
            ( json_returning_clause )?
            ( json_behavior_clause )?
        ')'
    */

    if !matches!(stream.peek2(), Ok((K(JsonValue), Op(OpenParenthesis)))) {
        return no_match(stream);
    }

    let (value, _, path_spec, passing, output, behavior) = skip_prefix(1, between_paren((
        json_value_expr,
        Comma,
        a_expr,
        json_passing_clause.optional(),
        json_returning_clause.optional(),
        json_behavior_clause.optional(),
    ))).parse(stream)?;

    let mut func = JsonValueFunc::new(value, path_spec);
    func.set_passing(passing)
        .set_output(output)
        .set_behavior(behavior.unwrap_or_default());

    Ok(func)
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
        pg_ast::JsonOutput,
        pg_ast::JsonValueExpr,
        pg_ast::TypeName::Int4,
        scan::Error::NoMatch,
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
    #[test_case("json_value" => matches Err(NoMatch(_)))]
    #[test_case("json_value 1" => matches Err(NoMatch(_)))]
    fn test_json_value_func(source: &str) -> scan::Result<JsonValueFunc> {
        test_parser!(source, json_value_func)
    }
}

use crate::combinators::expr::a_expr;
use crate::combinators::foundation::between_paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_behavior_clause;
use crate::combinators::json_passing_clause;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::JsonValueFunc;
use pg_lexer::Keyword::JsonValue;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::OpenParenthesis;
