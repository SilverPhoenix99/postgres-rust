#[derive(Debug, PartialEq, Eq, From)]
pub(super) enum JsonAggFunc {
    Array(JsonArrayAgg),
    Object(JsonObjectAgg),
}

impl From<JsonAggFunc> for FuncExprWindowless {
    fn from(value: JsonAggFunc) -> Self {
        match value {
            JsonAggFunc::Array(func) => FuncExprWindowless::SqlFunction(func.into()),
            JsonAggFunc::Object(func) => FuncExprWindowless::SqlFunction(func.into()),
        }
    }
}

pub(super) fn json_aggregate_func(stream: &mut TokenStream) -> scan::Result<JsonAggFunc> {
    use crate::stream::TokenValue::Keyword as K;
    use crate::stream::TokenValue::Operator as Op;

    /*
          json_objectagg
        | json_arrayagg
    */

    // Both 1st keywords are ColumnName, and they conflict with `func_application` and `prefixed_expr`,
    // so peeking is needed.

    match stream.peek2()? {
        (K(JsonObjectagg), Op(OpenParenthesis)) => json_objectagg(stream).map(From::from),
        (K(JsonArrayagg), Op(OpenParenthesis)) => json_arrayagg(stream).map(From::from),
        _ => no_match(stream)
    }
}

fn json_objectagg(stream: &mut TokenStream) -> scan::Result<JsonObjectAgg> {

    /*
        JSON_OBJECTAGG '('
            json_name_and_value
            ( json_constructor_null_clause )? // defaults to `false`
            ( json_key_uniqueness_constraint )? // defaults to `false`
            ( json_returning_clause )?
        ')'
    */

    let (_, (arg, absent_on_null, unique, output)) = seq!(
        skip(1),
        paren!(seq!(
            json_name_and_value,
            json_constructor_null_clause.optional(),
            json_key_uniqueness_constraint.optional(),
            json_returning_clause.optional()
        ))
    ).parse(stream)?;

    let func = JsonObjectAgg::new(
        arg,
        output,
        unique.unwrap_or_default(),
        absent_on_null.unwrap_or_default()
    );

    Ok(func)
}

fn json_arrayagg(stream: &mut TokenStream) -> scan::Result<JsonArrayAgg> {

    /*
        JSON_ARRAYAGG '('
            json_value_expr
            ( sort_clause )?
            ( json_constructor_null_clause )? // defaults to `true`
            ( json_returning_clause )?
        ')'
    */

    let (_, (arg, sort, absent_on_null, output)) = seq!(
        skip(1),
        paren!(seq!(
            json_value_expr,
            sort_clause.optional(),
            json_constructor_null_clause.optional(),
            json_returning_clause.optional()
        ))
    ).parse(stream)?;

    let func = JsonArrayAgg::new(
        arg,
        output,
        absent_on_null.unwrap_or(true),
        sort.map(|(sort, _)| sort)
    );

    Ok(func)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        crate::scan::Error::{Eof, NoMatch},
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::JsonKeyValue,
        pg_ast::JsonOutput,
        pg_ast::JsonValueExpr,
        pg_ast::SortBy,
        pg_ast::TypeName::{Int4, Int8},
    };

    #[test_case("json_objectagg('foo': 1)" => Ok(JsonAggFunc::Object(
        JsonObjectAgg::new(
            JsonKeyValue::new(
                StringConst("foo".into()),
                JsonValueExpr::from(IntegerConst(1))
            ),
            None,
            false,
            false
        )
    )))]
    #[test_case("json_objectagg('bar': 2 absent on null with unique returning int)" => Ok(JsonAggFunc::Object(
        JsonObjectAgg::new(
            JsonKeyValue::new(
                StringConst("bar".into()),
                JsonValueExpr::from(IntegerConst(2))
            ),
            Some(JsonOutput::from(Int4)),
            true,
            true
        )
    )))]
    #[test_case("json_arrayagg(1)" => Ok(JsonAggFunc::Array(
        JsonArrayAgg::new(
            JsonValueExpr::from(IntegerConst(1)),
            None,
            true,
            None
        )
    )))]
    #[test_case("json_arrayagg(2 order by 3 null on null returning bigint)" => Ok(JsonAggFunc::Array(
        JsonArrayAgg::new(
            JsonValueExpr::from(IntegerConst(2)),
            Some(JsonOutput::from(Int8)),
            false,
            Some(vec![SortBy::new(
                IntegerConst(3),
                None,
                None
            )])
        )
    )))]
    #[test_case("json_objectagg" => matches Err(Eof(_)))]
    #[test_case("json_objectagg 1" => matches Err(NoMatch(_)))]
    #[test_case("json_arrayagg" => matches Err(Eof(_)))]
    #[test_case("json_arrayagg 1" => matches Err(NoMatch(_)))]
    fn test_json_aggregate_func(source: &str) -> scan::Result<JsonAggFunc> {
        test_parser!(source, json_aggregate_func)
    }
}

use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_constructor_null_clause;
use crate::combinators::json_key_uniqueness_constraint;
use crate::combinators::json_name_and_value;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use crate::combinators::sort_clause;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use derive_more::From;
use pg_ast::FuncExprWindowless;
use pg_ast::JsonArrayAgg;
use pg_ast::JsonObjectAgg;
use pg_lexer::Keyword::JsonArrayagg;
use pg_lexer::Keyword::JsonObjectagg;
use pg_lexer::OperatorKind::OpenParenthesis;
