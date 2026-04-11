pub(super) fn paren_table_ref(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
        '(' table_ref_paren ')' ( alias_clause )?
    */

    let (mut table_ref, alias) = seq!(
        paren!(table_ref_paren),
        alias_clause.optional()
    ).parse(ctx)?;

    if let Some(alias) = alias {
        table_ref = ParenTableRef::new(table_ref, alias).into()
    }

    Ok(table_ref)
}

fn table_ref_paren(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
          WITH | SELECT | TABLE => SelectStmt
        | (VALUES, '(') => SelectStmt
        | _ => table_ref
    */

    match ctx.stream_mut().peek2() {
        Ok(
            (Keyword(With | Select | Table), _)
            | (Keyword(Values), Operator(OpenParenthesis))
        ) => {
            select_stmt(ctx)
                .map(SubselectTableRef::new)
                .map(From::from)
        },

        _ => table_ref(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::RelationTableRef;
    use test_case::test_case;

    #[test_case("foo" => Ok(
        RelationTableRef::new("foo").into()
    ))]
    fn test_table_ref_paren(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, table_ref_paren)
    }

    #[ignore]
    #[test_case("select 1" => matches Ok(_))]
    #[test_case("table" => matches Ok(_))]
    #[test_case("values (1)" => matches Ok(_))]
    #[test_case("with (1) as t select * from t" => matches Ok(_))]
    fn test_table_ref_paren_select_stmt(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, table_ref_paren)
    }

    #[test_case("(foo)" => Ok(
        RelationTableRef::new("foo").into()
    ))]
    #[test_case("(bar) as qux" => Ok(
        ParenTableRef::new(
            RelationTableRef::new("bar"),
            "qux"
        ).into()
    ))]
    fn test_paren_table_ref(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, paren_table_ref)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::stmt::select_stmt;
use crate::combinators::table_ref;
use crate::combinators::table_ref::alias_clause;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::ParenTableRef;
use pg_ast::SubselectTableRef;
use pg_ast::TableRef;
use pg_lexer::Keyword::Select;
use pg_lexer::Keyword::Table;
use pg_lexer::Keyword::Values;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
