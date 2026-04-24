pg_basics::reexport! {
    graph_pattern_quantifier,
    is_label_expression,
    path_factor,
    path_pattern_list,
    path_primary,
    path_primary_expr,
    path_term,
}

pub(super) fn graph_pattern(ctx: &mut ParserContext) -> scan::Result<GraphPattern> {

    /*
        path_pattern_list ( where_clause )?
    */

    let (path_pattern_list, where_clause) = seq!(
        path_pattern_list,
        where_clause.optional()
    ).parse(ctx)?;

    let mut pattern = GraphPattern::default();
    pattern.set_path_patterns(path_pattern_list)
        .set_where_clause(where_clause);

    Ok(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::BooleanConst;
    use pg_ast::GraphElementPattern;
    use pg_ast::GraphElementPatternKind;
    use test_case::test_matrix;

    #[test_matrix("-[]->" => Ok(
        GraphPattern::default()
            .with_path_patterns(vec![vec![
                GraphElementPatternKind::EdgePatternRight(
                    GraphElementPattern::default()
                )
            ]])
    ))]
    #[test_matrix("-[]-> where true" => Ok(
        GraphPattern::default()
            .with_path_patterns(vec![vec![
                GraphElementPatternKind::EdgePatternRight(
                    GraphElementPattern::default()
                )
            ]])
            .with_where_clause(BooleanConst(true))
    ))]
    fn test_graph_pattern(source: &str) -> scan::Result<GraphPattern> {
        test_parser!(source, graph_pattern)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::where_clause;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::GraphPattern;
use pg_parser_core::scan;
