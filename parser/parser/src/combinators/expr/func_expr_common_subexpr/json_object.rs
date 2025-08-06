pub(super) fn json_object(stream: &mut TokenStream) -> scan::Result<JsonObjectExpr> {

    /*
        JSON_OBJECT '(' ( json_object_args )? ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(
        json_object_args.optional()
    )).parse(stream)?;

    let expr = expr.unwrap_or_default();
    Ok(expr)
}

fn json_object_args(stream: &mut TokenStream) -> scan::Result<JsonObjectExpr> {

    /*
          json_returning_clause
        | func_arg_list
        | json_name_and_value_list
            ( json_object_constructor_null_clause )?
            ( json_key_uniqueness_constraint )?
            ( json_returning_clause )?
    */

    if let K(Returning) = stream.peek()? {
        let output = json_returning_clause(stream)?;
        let expr = JsonObjectArgs::new()
            .with_output(output);
        return Ok(SqlSyntax(expr));
    }

    let (first, _) = func_arg_expr(stream)?;
    if
        first.name().is_some()
        || ! matches!(stream.peek(), Ok(K(Value) | Op(Colon)))
    {
        // ExplicitCall

        let args = seq!(Comma, func_arg_list)
            .parse(stream)
            .optional()?;

        let args = match args {
            None => vec![first],
            Some((_, args)) => {

                let args = args.into_iter()
                    .map(|(arg, _)| arg);

                iter::once(first)
                    .chain(args)
                    .collect()
            }
        };

        return Ok(ExplicitCall(Some(args)))
    }

    let (_, json_value) = seq!(
        alt!(
            Value.skip(),
            Colon.skip()
        ),
        json_value_expr
    ).parse(stream)
        .required()?;

    let (_, key) = first.into();
    let first = JsonKeyValue::new(key, json_value);

    let (exprs, absent_on_null, unique, output) = seq!(
        seq!(Comma, json_name_and_value_list).optional(),
        json_constructor_null_clause.optional(),
        json_key_uniqueness_constraint.optional(),
        json_returning_clause.optional(),
    ).parse(stream)?;

    let exprs = match exprs {
        None => vec![first],
        Some((_, mut exprs)) => {
            exprs.insert(0, first);
            exprs
        }
    };

    let mut expr = JsonObjectArgs::new();
    expr.set_exprs(Some(exprs))
        .set_output(output)
        .set_unique(unique.unwrap_or_default())
        .set_absent_on_null(absent_on_null.unwrap_or_default());

    Ok(SqlSyntax(expr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_ast::JsonObjectExpr;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::JsonOutput,
        pg_ast::JsonValueExpr,
        pg_ast::NamedValue,
        pg_ast::TypeName::Int4,
    };

    #[test_case("json_object()" => Ok(ExplicitCall(None)) ; "json_object_with_empty_args")]
    #[test_case("json_object('foo')" => matches Ok(ExplicitCall(Some(_))))]
    fn test_json_object(source: &str) -> scan::Result<JsonObjectExpr> {
        test_parser!(source, json_object)
    }

    #[test_case("returning int" => Ok(SqlSyntax(
        JsonObjectArgs::new()
            .with_output(JsonOutput::from(Int4))
    )))]
    #[test_case("1, foo := 2, bar => 3" => Ok(ExplicitCall(Some(vec![
        NamedValue::unnamed(IntegerConst(1)),
        NamedValue::new(Some("foo".into()), IntegerConst(2)),
        NamedValue::new(Some("bar".into()), IntegerConst(3)),
    ]))))]
    #[test_case("'bar': 2" => Ok(SqlSyntax(
        JsonObjectArgs::new()
            .with_exprs(vec![
                JsonKeyValue::new(
                    StringConst("bar".into()),
                    JsonValueExpr::from(IntegerConst(2))
                )
            ])

    )))]
    #[test_case("'baz' value 3 absent on null with unique keys returning int" => Ok(SqlSyntax(
        JsonObjectArgs::new()
            .with_exprs(vec![
                JsonKeyValue::new(
                    StringConst("baz".into()),
                    JsonValueExpr::from(IntegerConst(3))
                )
            ])
            .with_absent_on_null(true)
            .with_unique(true)
            .with_output(JsonOutput::from(Int4))
    )))]
    fn test_json_object_args(source: &str) -> scan::Result<JsonObjectExpr> {
        test_parser!(source, json_object_args)
    }
}

use crate::combinators::func_arg_expr;
use crate::combinators::func_arg_list;
use crate::combinators::json_constructor_null_clause;
use crate::combinators::json_key_uniqueness_constraint;
use crate::combinators::json_name_and_value_list;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use core::iter;
use pg_ast::JsonKeyValue;
use pg_ast::JsonObjectArgs;
use pg_ast::JsonObjectExpr;
use pg_ast::JsonObjectExpr::ExplicitCall;
use pg_ast::JsonObjectExpr::SqlSyntax;
use pg_combinators::alt;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::skip;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Returning;
use pg_lexer::Keyword::Value;
use pg_lexer::OperatorKind::Colon;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::stream::TokenValue::Keyword as K;
use pg_parser_core::stream::TokenValue::Operator as Op;
use pg_parser_core::Optional;
use pg_parser_core::Required;
