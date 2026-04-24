pub(super) fn json_array(ctx: &mut ParserContext) -> scan::Result<SqlFunction> {

    /*
          JSON_ARRAY '(' SelectStmt ( json_format_clause )? ( json_returning_clause )? ')'
        | JSON_ARRAY '(' json_value_expr_list ( json_constructor_null_clause )? ( json_returning_clause )? ')'
        | JSON_ARRAY '(' json_returning_clause ')'
        | JSON_ARRAY '(' ')'
    */

    let (_, (params, output)) = seq!(JsonArray, paren!(seq!(
        json_array_params.optional(),
        json_returning_clause.optional()
    ))).parse(ctx)?;

    let func = match params {
        Some(Query(mut ctor)) => {
            ctor.set_output(output);
            ctor.into()
        }
        Some(Values(mut ctor)) => {
            ctor.set_output(output);
            ctor.into()
        }
        None => JsonArrayEmpty(output)
    };

    Ok(func)
}

#[derive(Debug, PartialEq, Eq)]
enum Params {
    Query(JsonArrayQueryConstructor),
    Values(JsonArrayConstructor),
}

fn json_array_params(ctx: &mut ParserContext) -> scan::Result<Params> {

    /*
          SelectStmt ( json_format_clause )?
        | json_value_expr_list ( json_constructor_null_clause )?
    */

    if is_select_stmt(ctx) {
        let (stmt, format) = seq!(select_stmt, json_format_clause.optional()).parse(ctx)?;
        let mut func = JsonArrayQueryConstructor::new(stmt);
        func.set_format(format);
        return Ok(Query(func));
    }

    let (values, absent_on_null) = seq!(
        json_value_expr_list,
        json_constructor_null_clause.optional()
    ).parse(ctx)?;

    let absent_on_null = absent_on_null.unwrap_or(true);

    let func = JsonArrayConstructor::new(values)
        .with_absent_on_null(absent_on_null);

    Ok(Values(func))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::JsonFormat;
    use pg_ast::JsonOutput;
    use pg_ast::JsonValueExpr;
    use pg_ast::TypeName;
    use test_case::test_matrix;

    #[test_matrix("select 1 format json" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    #[test_matrix("select 1" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    #[test_matrix("1 format json, 2 null on null" => Ok(Values(
        JsonArrayConstructor::new(vec![
            JsonValueExpr::new(IntegerConst(1))
                .with_format(JsonFormat::text()),
            JsonValueExpr::new(IntegerConst(2))
        ])
        .with_absent_on_null(false)
    )))]
    fn test_json_array_params(source: &str) -> scan::Result<Params> {
        test_parser!(source, json_array_params)
    }

    #[test_matrix("json_array()" => Ok(
        JsonArrayEmpty(None)
    ))]
    #[test_matrix("json_array(returning int)" => Ok(
        JsonArrayEmpty(Some(
            JsonOutput::new(TypeName::Int4)
        ))
    ))]
    #[test_matrix("json_array(1)" => Ok(
        JsonArrayConstructor::new(vec![
            JsonValueExpr::new(IntegerConst(1))
        ])
        .into()
    ))]
    #[test_matrix("json_array(2 returning int)" => Ok(
        JsonArrayConstructor::new(vec![
            JsonValueExpr::new(IntegerConst(2))
        ])
        .with_output(
            JsonOutput::new(TypeName::Int4)
        )
        .into()
    ))]
    #[test_matrix("json_array(select 1)" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    #[test_matrix("json_array(select 2 returning int)" => ignore["select_stmt not implemented yet"] matches Ok(_))]
    fn test_json_array(source: &str) -> scan::Result<SqlFunction> {
        test_parser!(source, json_array)
    }
}

use self::Params::Query;
use self::Params::Values;
use crate::combinators::core::Combinator;
use crate::combinators::json_constructor_null_clause;
use crate::combinators::json_format_clause;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr_list;
use crate::combinators::stmt::is_select_stmt;
use crate::combinators::stmt::select_stmt;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::JsonArrayConstructor;
use pg_ast::JsonArrayQueryConstructor;
use pg_ast::SqlFunction;
use pg_ast::SqlFunction::JsonArrayEmpty;
use pg_lexer::Keyword::JsonArray;
use pg_parser_core::scan;
