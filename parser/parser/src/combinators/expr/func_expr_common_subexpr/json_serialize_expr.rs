pub(super) fn json_serialize_expr(ctx: &mut ParserContext) -> scan::Result<JsonSerializeExpr> {

    /*
        JSON_SERIALIZE '(' json_value_expr ( json_returning_clause )? ')'
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

    let (_, (value, output)) = seq!(
        skip(1),
        paren!(seq!(
            json_value_expr,
            json_returning_clause.optional(),
        ))
    ).parse(ctx)?;

    let mut expr = JsonSerializeExpr::new(value);
    expr.set_output(output);

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::JsonOutput;
    use pg_ast::JsonValueExpr;
    use pg_ast::TypeName::Int4;
    use test_case::test_matrix;

    #[test_matrix("json_serialize(1)" => Ok(JsonSerializeExpr::new(
        JsonValueExpr::from(IntegerConst(1))
    )))]
    #[test_matrix("json_serialize(1 returning int)" => Ok(
        JsonSerializeExpr::new(JsonValueExpr::from(IntegerConst(1)))
            .with_output(JsonOutput::from(Int4))
    ))]
    fn test_json_serialize_expr(source: &str) -> scan::Result<JsonSerializeExpr> {
        test_parser!(source, json_serialize_expr)
    }
}

use crate::combinators::core::skip;
use crate::combinators::core::Combinator;
use crate::combinators::json_returning_clause;
use crate::combinators::json_value_expr;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::JsonSerializeExpr;
use pg_parser_core::scan;
