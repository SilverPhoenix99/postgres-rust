pg_basics::reexport! {
    alias_clause,
    func_alias_clause,
    json_table,
    lateral_func_table,
    rowsfrom_list,
    select_table_ref,
    tablesample_table_ref,
    xmltable,
}

fn table_ref_1(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
          LATERAL select_table_ref
        | LATERAL lateral_func_table
        | lateral_func_table
        | tablesample_table_ref
    */

    alt!(
        seq!(
            Lateral,
            alt!(
                select_table_ref.map(From::from),
                lateral_func_table
            )
        ).map(|(_, table_ref)| match table_ref {
            TableRef::XmlTable(table_ref) => table_ref.with_lateral(true).into(),
            TableRef::JsonTable(table_ref) => table_ref.with_lateral(true).into(),
            TableRef::Rows(table_ref) => table_ref.with_lateral(true).into(),
            TableRef::Function(table_ref) => table_ref.with_lateral(true).into(),
            TableRef::Subselect(table_ref) => table_ref.with_lateral(true).into(),
            _ => unreachable!(),
        }),
        lateral_func_table,
        tablesample_table_ref.map(From::from),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[ignore]
    #[test_case("(select 1)" => matches Ok(_))]
    fn test_table_ref_1_subselect_table_ref(source: &str) -> scan::Result<TableRef> {
        // TODO: merge with test_table_ref_1, when subselect_stmt is working
        test_parser!(source, table_ref_1)
    }

    #[test_case("lateral foo()" => matches Ok(_))]
    #[test_case("foo()" => matches Ok(_))]
    #[test_case("bar" => matches Ok(_))]
    fn test_table_ref_1(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, table_ref_1)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::TableRef;
use pg_lexer::Keyword::Lateral;
use pg_parser_core::scan;
