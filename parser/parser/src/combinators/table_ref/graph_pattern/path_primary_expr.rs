pub(super) fn path_primary_expr(ctx: &mut ParserContext) -> scan::Result<GraphElementPattern> {

    /*
        ( ColId )? ( is_label_expression )? ( where_clause )?
    */

    let (variable, label_expr, where_clause) = seq!(
        col_id.optional(),
        is_label_expression.optional(),
        where_clause.optional()
    ).parse(ctx)?;

    let mut pattern = GraphElementPattern::default();
    pattern.set_variable(variable)
        .set_label_expr(label_expr)
        .set_where_clause(where_clause);

    Ok(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::BooleanConst;
    use test_case::test_case;

    #[test_case("" => Ok(GraphElementPattern::default()))]
    #[test_case("foo" => Ok(
        GraphElementPattern::default()
            .with_variable("foo")
    ))]
    #[test_case("bar is a | b" => Ok(
        GraphElementPattern::default()
            .with_variable("bar")
            .with_label_expr(vec!["a".into(), "b".into()])
    ))]
    #[test_case("baz where true" => Ok(
        GraphElementPattern::default()
            .with_variable("baz")
            .with_where_clause(BooleanConst(true))
    ))]
    #[test_case("qux is c | d where true" => Ok(
        GraphElementPattern::default()
            .with_variable("qux")
            .with_label_expr(vec!["c".into(), "d".into()])
            .with_where_clause(BooleanConst(true))
    ))]
    #[test_case("is e" => Ok(
        GraphElementPattern::default()
            .with_label_expr(vec!["e".into()])
    ))]
    #[test_case("is f where true" => Ok(
        GraphElementPattern::default()
            .with_label_expr(vec!["f".into()])
            .with_where_clause(BooleanConst(true))
    ))]
    #[test_case("where false" => Ok(
        GraphElementPattern::default()
            .with_where_clause(BooleanConst(false))
    ))]
    fn test_path_primary_expr(source: &str) -> scan::Result<GraphElementPattern> {
        test_parser!(source, path_primary_expr)
    }
}

use super::is_label_expression;
use crate::combinators::col_id::col_id;
use crate::combinators::core::Combinator;
use crate::combinators::where_clause;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::GraphElementPattern;
use pg_parser_core::scan;
