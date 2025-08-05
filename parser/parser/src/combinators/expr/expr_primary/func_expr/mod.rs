pg_basics::reexport! { pub(super)
    filter_clause,
    over_clause,
    within_group_clause,
}

pub(super) fn func_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
          func_expr_common_subexpr
        | json_aggregate_func filter_clause over_clause
    */

    alt!(
        func_expr_common_subexpr.map(From::from),
        json_agg_func
    ).parse(stream)
}

fn json_agg_func(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        json_aggregate_func filter_clause over_clause
    */

    let (func, filter, over_clause) = seq!(
        json_aggregate_func,
        filter_clause.optional(),
        over_clause.optional()
    ).parse(stream)?;

    let expr = match func {
        JsonAggFunc::Array(agg) => JsonArrayAggExpr::new(agg, filter, over_clause).into(),
        JsonAggFunc::Object(agg) => JsonObjectAggExpr::new(agg, filter, over_clause).into(),
    };

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
        pg_ast::JsonArrayAgg,
        pg_ast::JsonKeyValue,
        pg_ast::JsonObjectAgg,
        pg_ast::JsonValueExpr,
    };

    #[test_case("json_arrayagg(1)" => Ok(
        JsonArrayAggExpr::new(
            JsonArrayAgg::new(
                JsonValueExpr::from(IntegerConst(1)),
                None,
                true,
                None
            ),
            None,
            None
        ).into()
    ))]
    #[test_case("json_objectagg('foo': 1) where 2 over foo" => Ok(
        JsonObjectAggExpr::new(
            JsonObjectAgg::new(
                JsonKeyValue::new(
                    StringConst("foo".into()),
                    JsonValueExpr::from(IntegerConst(1)),
                ),
                None,
                false,
                false
            ),
            None,
            None
        ).into()
    ))]
    // These only quickly check that statements aren't missing:
    #[test_case("collation for (5)" => matches Ok(_))]
    fn test_func_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, func_expr)
    }
}

use crate::combinators::expr::func_expr_common_subexpr;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::json_aggregate_func;
use crate::combinators::JsonAggFunc;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_ast::JsonArrayAggExpr;
use pg_ast::JsonObjectAggExpr;
