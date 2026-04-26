pub(super) fn tablesample_table_ref(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
        non_inherited_relation_expr ( alias_clause )? ( tablesample_clause )?
    */

    let (relation, alias, tablesample) = seq!(
        non_inherited_relation_expr,
        alias_clause.optional(),
        tablesample_clause.optional()
    ).parse(ctx)?;

    let (name, inherited) = relation.into();
    let mut table_ref = RelationTableRef::new(name)
        .with_inherited(inherited);

    table_ref.set_alias(alias);

    if let Some(SampleClause { function_name, args, repeatable_expr }) = tablesample {

        let mut table_ref = SampleTableRef::new(table_ref, function_name, args);
        table_ref.set_repeatable(repeatable_expr);

        Ok(table_ref.into())
    }
    else {
        Ok(table_ref.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("only(foo) as t tablesample f(1)" => Ok(
        SampleTableRef::new(
            RelationTableRef::new("foo")
                .with_inherited(false)
                .with_alias("t"),
            vec!["f".into()],
            vec![IntegerConst(1)]
        )
        .into()
    ))]
    #[test_matrix("only bar as s" => Ok(
        RelationTableRef::new("bar")
            .with_inherited(false)
            .with_alias("s")
            .into()
    ))]
    #[test_matrix("only(baz)" => Ok(
        RelationTableRef::new("baz")
            .with_inherited(false)
            .into()
    ))]
    #[test_matrix("only qux tablesample g(2)" => Ok(
        SampleTableRef::new(
            RelationTableRef::new("qux")
                .with_inherited(false),
            vec!["g".into()],
            vec![IntegerConst(2)]
        )
        .into()
    ))]
    fn test_tablesample_table_ref(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, tablesample_table_ref)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::relation_expr::non_inherited_relation_expr;
use crate::combinators::table_ref::alias_clause;
use crate::combinators::table_ref::tablesample_clause;
use crate::combinators::table_ref::SampleClause;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::RelationTableRef;
use pg_ast::SampleTableRef;
use pg_ast::TableRef;
use pg_parser_core::scan;
