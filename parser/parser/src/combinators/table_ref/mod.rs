pg_basics::reexport! {
    alias_clause,
    ambiguous_table_ref,
    func_alias_clause,
    func_subexpr_table_ref,
    func_windowless_table_ref,
    graph_pattern,
    graph_table_ref,
    join_qual,
    join_type,
    join_type_prefix,
    joined_table_right,
    json_aggregate_table_ref,
    json_table,
    lateral_table_ref,
    ordinality,
    paren_table_ref,
    rowsfrom_list,
    select_table_ref,
    table_ref_primary,
    tablesample_clause,
    tablesample_table_ref,
    xmltable,
}

pub(super) fn table_ref(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
        table_ref_primary ( joined_table_rhs )*
    */

    let mut table_ref = table_ref_primary(ctx)?;

    while let Some(right_side) = joined_table_right(ctx).optional()? {

        let JoinRightSide {
            table_ref: right_table_ref,
            join_kind,
            alias
        } = right_side;

        let mut join = JoinExpr::new(join_kind, table_ref, right_table_ref);
        join.set_alias(alias);

        table_ref = join.into();
    }

    Ok(table_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::JoinKind;
    use pg_ast::JoinQual;
    use pg_ast::RelationTableRef;
    use pg_ast::TableRef;
    use pg_parser_core::scan;
    use test_case::test_matrix;

    #[test_matrix("foo" => Ok(
        RelationTableRef::new("foo").into()
    ))]
    #[test_matrix("bar natural join qux" => Ok(
        JoinExpr::new(
            JoinKind::Inner(Some(JoinQual::Natural)),
            RelationTableRef::new("bar"),
            RelationTableRef::new("qux")
        ).into()
    ))]
    #[test_matrix("a cross join b left join c using(d)" => Ok(
        JoinExpr::new(
            JoinKind::Left(
                JoinQual::Using(
                    vec!["d".into()]
                )
            ),
            JoinExpr::new(
                JoinKind::cross_join(),
                RelationTableRef::new("a"),
                RelationTableRef::new("b")
            ),
            RelationTableRef::new("c")
        ).into()
    ))]
    #[test_matrix("a cross join b left join c using(d) join (e cross join f) using(g) as h" => Ok(
        JoinExpr::new(
            JoinKind::Inner(Some(
                JoinQual::Using(
                    vec!["g".into()]
                )
            )),
            JoinExpr::new(
                JoinKind::Left(
                    JoinQual::Using(
                        vec!["d".into()]
                    )
                ),
                JoinExpr::new(
                    JoinKind::cross_join(),
                    RelationTableRef::new("a"),
                    RelationTableRef::new("b")
                ),
                RelationTableRef::new("c")
            ),
            JoinExpr::new(
                JoinKind::cross_join(),
                RelationTableRef::new("e"),
                RelationTableRef::new("f")
            )
        )
        .with_alias("h")
        .into()
    ))]
    fn test_table_ref(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, table_ref)
    }
}

use crate::context::ParserContext;
use pg_ast::JoinExpr;
use pg_ast::TableRef;
use pg_parser_core::scan;
use pg_parser_core::Optional;
