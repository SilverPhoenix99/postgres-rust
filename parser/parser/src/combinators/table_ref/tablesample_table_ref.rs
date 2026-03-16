pub(super) fn tablesample_table_ref(ctx: &mut ParserContext) -> scan::Result<SampleTableRef> {

    /*
        relation_expr ( alias_clause )? ( tablesample_clause )?
    */

    let (relation, alias, tablesample) = seq!(
        relation_expr,
        alias_clause.optional(),
        tablesample_clause.optional()
    ).parse(ctx)?;

    let mut table_ref = SampleTableRef::new(relation);
    table_ref.set_alias(alias)
        .set_table_sample(tablesample);

    Ok(table_ref)
}

fn tablesample_clause(ctx: &mut ParserContext) -> scan::Result<TableSample> {

    /*
        TABLESAMPLE func_name '(' expr_list ')' ( REPEATABLE '(' a_expr ')' )?
    */

    let (_, name, params, repeatable_expr) = seq!(
        Tablesample,
        func_name,
        paren!(expr_list),
        repeatable_clause.optional()
    ).parse(ctx)?;

    let mut table_sample = TableSample::new(name, params);
    table_sample.set_repeatable(repeatable_expr);

    Ok(table_sample)
}

/// Alias: `opt_repeatable_clause`
fn repeatable_clause(ctx: &mut ParserContext) -> scan::Result<ExprNode> {

    /*
        REPEATABLE '(' a_expr ')'
    */

    let (_, expr) = seq!(Repeatable, paren!(a_expr)).parse(ctx)?;

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::IntegerConst,
        RelationExpr
    };
    use test_case::test_case;

    #[test_case("foo as t tablesample f(1)" => Ok(
        SampleTableRef::new("foo")
            .with_alias("t")
            .with_table_sample(
                TableSample::new(
                    vec!["f".into()],
                    vec![IntegerConst(1)],
                )
            )
    ))]
    #[test_case("bar as s" => Ok(
        SampleTableRef::new("bar")
            .with_alias("s")
    ))]
    #[test_case("baz tablesample g(2)" => Ok(
        SampleTableRef::new("baz")
            .with_table_sample(
                TableSample::new(
                    vec!["g".into()],
                    vec![IntegerConst(2)],
                )
            )
    ))]
    fn test_tablesample_table_ref(source: &str) -> scan::Result<SampleTableRef> {
        test_parser!(source, tablesample_table_ref)
    }

    #[test_case("tablesample foo(1) repeatable (10)" => Ok(
        TableSample::new(
            vec!["foo".into()],
            vec![IntegerConst(1)],
        )
        .with_repeatable(IntegerConst(10))
    ))]
    #[test_case("tablesample bar(2)" => Ok(
        TableSample::new(
            vec!["bar".into()],
            vec![IntegerConst(2)],
        )
    ))]
    fn test_tablesample_clause(source: &str) -> scan::Result<TableSample> {
        test_parser!(source, tablesample_clause)
    }

    #[test_case("repeatable (1)" => Ok(ExprNode::IntegerConst(1)))]
    fn test_repeatable_clause(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, repeatable_clause)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::combinators::expr_list;
use crate::combinators::func_name;
use crate::combinators::relation_expr;
use crate::combinators::table_ref::alias_clause;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::ExprNode;
use pg_ast::SampleTableRef;
use pg_ast::TableSample;
use pg_lexer::Keyword::Repeatable;
use pg_lexer::Keyword::Tablesample;
use pg_parser_core::scan;
