pub(super) fn table_ref_primary(ctx: &mut ParserContext) -> scan::Result<TableRef> {

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
                select_table_ref.map(|table_ref| table_ref.with_lateral(true).into()),
                lateral_table_ref.map(|table_ref| match table_ref {
                    XmlTable(table_ref) => table_ref.with_lateral(true).into(),
                    JsonTable(table_ref) => table_ref.with_lateral(true).into(),
                    Rows(table_ref) => table_ref.with_lateral(true).into(),
                    Function(table_ref) => table_ref.with_lateral(true).into(),
                    _ => unreachable!(),
                })
            )
        ).map(|(_, table_ref)| table_ref),
        lateral_table_ref,
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
    fn test_table_ref_primary_subselect_table_ref(source: &str) -> scan::Result<TableRef> {
        // TODO: merge with test_table_ref_primary, when subselect_stmt is working
        test_parser!(source, table_ref_primary)
    }

    #[test_case("lateral foo()" => matches Ok(_))]
    #[test_case("foo()" => matches Ok(_))]
    #[test_case("bar" => matches Ok(_))]
    fn test_table_ref_primary(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, table_ref_primary)
    }
}

use super::lateral_table_ref;
use super::tablesample_table_ref;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::table_ref::select_table_ref;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::TableRef;
use pg_lexer::Keyword::Lateral;
use pg_parser_core::scan;
use TableRef::Function;
use TableRef::JsonTable;
use TableRef::Rows;
use TableRef::XmlTable;
