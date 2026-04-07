pub(super) fn table_ref_primary(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
          TODO: '(' table_ref_paren ')' ( alias_clause )?
        | TODO: GRAPH_TABLE '(' qualified_name MATCH graph_pattern COLUMNS '(' labeled_expr_list ')' ')' ( alias_clause )?
        | LATERAL '(' SelectStmt ')' ( alias_clause )?
        | LATERAL lateral_table_ref
        | LATERAL func_expr_windowless ( ordinality )? ( func_alias_clause )?
        | lateral_table_ref
        | func_expr_common_subexpr ( ordinality )? ( func_alias_clause )?
        | json_aggregate_func ( ordinality )? ( func_alias_clause )?
        | TODO: non_inherited_relation_expr ( alias_clause )? ( tablesample_clause )?
        | TODO: ambiguous_table_ref
    */

    alt!(
        seq!(
            Lateral,
            alt!(
                select_table_ref.map(|table_ref|
                    table_ref.with_lateral(true).into()
                ),
                lateral_table_ref.map(|table_ref| match table_ref {
                    XmlTable(table_ref) => table_ref.with_lateral(true).into(),
                    JsonTable(table_ref) => table_ref.with_lateral(true).into(),
                    Rows(table_ref) => table_ref.with_lateral(true).into(),
                    _ => unreachable!(),
                }),
                // Needs to be last due to conflicts with `lateral_func_table`
                func_windowless_table_ref.map(|table_ref|
                    table_ref.with_lateral(true).into()
                )
            )
        ).map(|(_, table_ref)| table_ref),
        lateral_table_ref,
        func_subexpr_table_ref.map(From::from),
        json_aggregate_table_ref.map(From::from),
        tablesample_table_ref.map(From::from),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[ignore]
    #[test_case("lateral (select 1)" => matches Ok(_))]
    fn test_table_ref_primary_subselect_table_ref(source: &str) -> scan::Result<TableRef> {
        // TODO: merge with test_table_ref_primary, when subselect_stmt is working
        test_parser!(source, table_ref_primary)
    }

    #[test_case("lateral rows from ( foo() )" => matches Ok(_))]
    #[test_case("lateral baz()" => matches Ok(_))]
    #[test_case("rows from ( foo() )" => matches Ok(_))]
    #[test_case("current_time" => matches Ok(_))]
    #[test_case("json_arrayagg(1)" => matches Ok(_))]
    #[test_case("bar" => matches Ok(_))]
    fn test_table_ref_primary(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, table_ref_primary)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::table_ref::func_subexpr_table_ref;
use crate::combinators::table_ref::func_windowless_table_ref;
use crate::combinators::table_ref::json_aggregate_table_ref;
use crate::combinators::table_ref::lateral_table_ref;
use crate::combinators::table_ref::select_table_ref;
use crate::combinators::table_ref::tablesample_table_ref;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::TableRef;
use pg_ast::TableRef::JsonTable;
use pg_ast::TableRef::Rows;
use pg_ast::TableRef::XmlTable;
use pg_lexer::Keyword::Lateral;
use pg_parser_core::scan;
