pub(super) fn lateral_table_ref(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
          xmltable
        | json_table
        | rows_from_stmt
    */

    let table_ref = match ctx.stream_mut().peek_n::<2>()? {
        [Keyword(Xmltable), Operator(OpenParenthesis)] => xmltable(ctx)?.into(),
        [Keyword(Kw::JsonTable), Operator(OpenParenthesis)] => json_table(ctx)?.into(),
        [Keyword(Rows), Keyword(FromKw)] => rows_from_stmt(ctx)?.into(),
        _ => return no_match(ctx),
    };

    Ok(table_ref)
}

/// Alias: `func_table`
fn rows_from_stmt(ctx: &mut ParserContext) -> scan::Result<RowsTableRef> {

    /*
        ROWS FROM '(' rowsfrom_list ')' ( ordinality )? ( func_alias_clause )?
    */

    let (_, _, rows, ordinality, alias) = seq!(
        Rows,
        FromKw,
        paren!(rowsfrom_list),
        ordinality.optional(),
        func_alias_clause.optional()
    ).parse(ctx)?;

    let mut table_ref = RowsTableRef::new(rows);
    table_ref.set_ordinality(ordinality.is_some())
        .set_alias(alias);

    Ok(table_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::FuncAliasColumn;
    use pg_ast::FuncArgsKind;
    use pg_ast::FuncCall;
    use pg_ast::OneOrBoth::Both;
    use pg_ast::RangeFunction;
    use test_case::test_matrix;

    #[test_matrix("rows from ( foo() )" => Ok(
        RowsTableRef::new(
            vec![RangeFunction::new(FuncCall::new(
                vec!["foo".into()],
                FuncArgsKind::Empty { order_within_group: None }
            ))]
        )
    ))]
    #[test_matrix("rows from ( baz() ) with ordinality" => Ok(
        RowsTableRef::new(
            vec![RangeFunction::new(FuncCall::new(
                vec!["baz".into()],
                FuncArgsKind::Empty { order_within_group: None }
            ))]
        )
        .with_ordinality(true)
    ))]
    #[test_matrix("rows from ( qux() ) as t(x)" => Ok(
        RowsTableRef::new(
            vec![RangeFunction::new(FuncCall::new(
                vec!["qux".into()],
                FuncArgsKind::Empty { order_within_group: None }
            ))]
        )
        .with_alias(Both(
            "t".into(),
            vec![FuncAliasColumn::new("x")]
        ))
    ))]
    #[test_matrix("rows from ( foo() ) with ordinality as s(y)" => Ok(
        RowsTableRef::new(
            vec![RangeFunction::new(FuncCall::new(
                vec!["foo".into()],
                FuncArgsKind::Empty { order_within_group: None }
            ))]
        )
        .with_ordinality(true)
        .with_alias(Both(
            "s".into(),
            vec![FuncAliasColumn::new("y")]
        ))
    ))]
    fn test_rows_from_stmt(source: &str) -> scan::Result<RowsTableRef> {
        test_parser!(source, rows_from_stmt)
    }

    #[test_matrix("xmltable('foo' passing 'bar' columns qux int)" => matches Ok(_))]
    #[test_matrix("json_table('foo', 'bar' columns(qux for ordinality))" => matches Ok(_))]
    #[test_matrix("rows from ( foo(1), bar(*) )" => matches Ok(_))]
    fn test_lateral_table_ref(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, lateral_table_ref)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::table_ref::func_alias_clause;
use crate::combinators::table_ref::json_table;
use crate::combinators::table_ref::ordinality;
use crate::combinators::table_ref::rowsfrom_list;
use crate::combinators::table_ref::xmltable;
use crate::no_match;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::RowsTableRef;
use pg_ast::TableRef;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Rows;
use pg_lexer::Keyword::Xmltable;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
