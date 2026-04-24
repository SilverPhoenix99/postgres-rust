pub(super) fn json_object(ctx: &mut ParserContext) -> scan::Result<JsonObjectExpr> {

    /*
        JSON_OBJECT '(' ( json_object_args )? ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, expr) = seq!(skip(1), paren!(
        json_object_args.optional()
    )).parse(ctx)?;

    let expr = expr.unwrap_or_default();
    Ok(expr)
}

fn json_object_args(ctx: &mut ParserContext) -> scan::Result<JsonObjectExpr> {

    /*
          json_returning_clause
        | func_arg_list
        | json_name_and_value_list
            ( json_object_constructor_null_clause )?
            ( json_key_uniqueness_constraint )?
            ( json_returning_clause )?
    */

    if let K(Returning) = ctx.stream_mut().peek()? {
        let output = json_returning_clause(ctx)?;
        let expr = JsonObjectArgs::new()
            .with_output(output);
        return Ok(SqlSyntax(expr));
    }

    let Located(first, _) = func_arg_expr(ctx)?;
    if
        first.name().is_some()
        || ! matches!(ctx.stream_mut().peek(), Ok(K(Value) | Op(Colon)))
    {
        // ExplicitCall

        let args = seq!(Comma, func_arg_list)
            .parse(ctx)
            .optional()?;

        let args = match args {
            None => vec![first],
            Some((_, args)) => {

                let args = args.into_iter()
                    .map(|Located(arg, _)| arg);

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
    ).parse(ctx)
        .required()?;

    let (_, key) = first.into();
    let first = JsonKeyValue::new(key, json_value);

    let (exprs, absent_on_null, unique, output) = seq!(
        seq!(Comma, json_name_and_value_list).optional(),
        json_constructor_null_clause.optional(),
        json_key_uniqueness_constraint.optional(),
        json_returning_clause.optional(),
    ).parse(ctx)?;

    let exprs = match exprs {
        None => vec![first],
        Some((_, mut exprs)) => {
            exprs.insert(0, first);
            exprs
        }
    };

    let mut expr = JsonObjectArgs::new();
    expr.set_expressions(Some(exprs))
        .set_output(output)
        .set_unique(unique.unwrap_or_default())
        .set_absent_on_null(absent_on_null.unwrap_or_default());

    Ok(SqlSyntax(expr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::{IntegerConst, StringConst};
    use pg_ast::JsonObjectExpr;
    use pg_ast::JsonOutput;
    use pg_ast::JsonValueExpr;
    use pg_ast::NamedValue;
    use pg_ast::TypeName::Int4;
    use test_case::test_matrix;

    #[test_matrix("json_object()" => Ok(ExplicitCall(None)) ; "json_object_with_empty_args")]
    #[test_matrix("json_object('foo')" => matches Ok(ExplicitCall(Some(_))))]
    fn test_json_object(source: &str) -> scan::Result<JsonObjectExpr> {
        test_parser!(source, json_object)
    }

    #[test_matrix("returning int" => Ok(SqlSyntax(
        JsonObjectArgs::new()
            .with_output(JsonOutput::from(Int4))
    )))]
    #[test_matrix("1, foo := 2, bar => 3" => Ok(ExplicitCall(Some(vec![
        NamedValue::new(IntegerConst(1)),
        NamedValue::new(IntegerConst(2)).with_name("foo"),
        NamedValue::new(IntegerConst(3)).with_name("bar"),
    ]))))]
    #[test_matrix("'bar': 2" => Ok(SqlSyntax(
        JsonObjectArgs::new()
            .with_expressions(vec![
                JsonKeyValue::new(
                    StringConst("bar".into()),
                    JsonValueExpr::from(IntegerConst(2))
                )
            ])

    )))]
    #[test_matrix("'baz' value 3 absent on null with unique keys returning int" => Ok(SqlSyntax(
        JsonObjectArgs::new()
            .with_expressions(vec![
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

use crate::alt;
use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::func_arg_expr;
use crate::combinators::func_arg_list;
use crate::combinators::json_constructor_null_clause;
use crate::combinators::json_key_uniqueness_constraint;
use crate::combinators::json_name_and_value_list;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use core::iter;
use pg_ast::JsonKeyValue;
use pg_ast::JsonObjectArgs;
use pg_ast::JsonObjectExpr;
use pg_ast::JsonObjectExpr::ExplicitCall;
use pg_ast::JsonObjectExpr::SqlSyntax;
use pg_basics::Located;
use pg_lexer::Keyword::Returning;
use pg_lexer::Keyword::Value;
use pg_lexer::OperatorKind::Colon;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword as K;
use pg_parser_core::stream::TokenValue::Operator as Op;
use pg_parser_core::Optional;
use pg_parser_core::Required;
