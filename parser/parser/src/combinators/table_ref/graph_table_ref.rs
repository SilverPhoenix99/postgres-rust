pub(super) fn graph_table_ref(ctx: &mut ParserContext) -> scan::Result<GraphTableRef> {

    /*
        GRAPH_TABLE '(' qualified_name MATCH graph_pattern COLUMNS '(' labeled_expr_list ')' ')' ( alias_clause )?
    */

    let (_, (graph_name, _, graph_pattern, _, columns), alias) = seq!(
        GraphTable,
        paren!(seq!(
            qualified_name,
            Match,
            graph_pattern,
            Columns,
            paren!(labeled_expr_list)
        )),
        alias_clause.optional()
    ).parse(ctx)?;

    let mut table_ref = GraphTableRef::new(graph_name, graph_pattern, columns);
    table_ref.set_alias(alias);

    Ok(table_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ColumnRef::SingleName,
        ExprNode::ColumnRef,
        GraphElementPatternKind::EdgePatternRight,
        GraphPattern,
        NamedValue,
    };
    use test_case::test_case;

    #[test_case("graph_table (foo match -[]-> columns (bar))" => Ok(
        GraphTableRef::new(
            "foo",
            GraphPattern::new()
                .with_path_patterns(vec![vec![
                    EdgePatternRight(Default::default())
                ]]),
            vec![NamedValue::unnamed(ColumnRef(SingleName("bar".into())))]
        )
    ))]
    #[test_case("graph_table (baz match -> columns (qux)) as a" => Ok(
        GraphTableRef::new(
            "baz",
            GraphPattern::new()
                .with_path_patterns(vec![vec![
                    EdgePatternRight(Default::default())
                ]]),
            vec![NamedValue::unnamed(ColumnRef(SingleName("qux".into())))]
        )
        .with_alias("a")
    ))]
    fn test_graph_table_ref(source: &str) -> scan::Result<GraphTableRef> {
        test_parser!(source, graph_table_ref)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::labeled_expr_list;
use crate::combinators::qualified_name;
use crate::combinators::table_ref::alias_clause;
use crate::combinators::table_ref::graph_pattern;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::GraphTableRef;
use pg_lexer::Keyword::Columns;
use pg_lexer::Keyword::GraphTable;
use pg_lexer::Keyword::Match;
use pg_parser_core::scan;
