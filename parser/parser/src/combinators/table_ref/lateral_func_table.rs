/// Inlined: `func_table`
pub(super) fn lateral_func_table(ctx: &mut ParserContext) -> scan::Result<TableRef> {

    /*
          xmltable
        | json_table
        | rows_from_stmt
        | function_stmt
    */

    let table_ref = match ctx.stream_mut().peek2()? {
        (Keyword(Xmltable), Operator(OpenParenthesis)) => xmltable(ctx)?.into(),
        (Keyword(Kw::JsonTable), Operator(OpenParenthesis)) => json_table(ctx)?.into(),
        (Keyword(Rows), Keyword(FromKw)) => rows_from_stmt(ctx)?.into(),

        // The 1st keyword of the previous productions conflict with function_stmt,
        // so the 2nd token needed to be checked to disambiguate.
        _ => function_stmt(ctx)?.into(),
    };

    Ok(table_ref)
}

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

    let mut table_ref = RowsTableRef::new(rows)
        .with_ordinality(ordinality.is_some());

    table_ref.set_alias(alias);
    Ok(table_ref)
}

fn function_stmt(ctx: &mut ParserContext) -> scan::Result<FunctionTableRef> {

    /*
        func_expr_windowless ( ordinality )? ( func_alias_clause )?
    */

    let (function, ordinality, alias) = seq!(
                func_expr_windowless,
                ordinality.optional(),
                func_alias_clause.optional()
            ).parse(ctx)?;

    let mut table_ref = FunctionTableRef::new(function)
        .with_ordinality(ordinality.is_some());

    table_ref.set_alias(alias);
    Ok(table_ref)
}

/// Alias: `opt_ordinality`
fn ordinality(ctx: &mut ParserContext) -> scan::Result<()> {

    /*
        WITH ORDINALITY
    */

    seq!(With, Ordinality).parse(ctx)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::StringConst,
        FuncAliasColumn,
        FuncArgsKind,
        FuncCall,
        FuncExprWindowless,
        JsonTable,
        JsonTableColumnDefinition,
        JsonTablePathSpec,
        JsonValueExpr,
        OneOrBoth::Both,
        RangeFunction,
        TypeName::Int4,
        XmlTable,
        XmlTableColumn,
        XmlTableColumnDefinition,
    };
    use test_case::test_case;

    #[test_case("with ordinality" => Ok(()))]
    fn test_ordinality(source: &str) -> scan::Result<()> {
        test_parser!(source, ordinality)
    }

    #[test_case("rows from ( foo() )" => Ok(
        RowsTableRef::new(
            vec![RangeFunction::new(FuncCall::new(
                vec!["foo".into()],
                FuncArgsKind::Empty { order_within_group: None }
            ))]
        )
    ))]
    #[test_case("rows from ( baz() ) with ordinality" => Ok(
        RowsTableRef::new(
            vec![RangeFunction::new(FuncCall::new(
                vec!["baz".into()],
                FuncArgsKind::Empty { order_within_group: None }
            ))]
        )
        .with_ordinality(true)
    ))]
    #[test_case("rows from ( qux() ) as t(x)" => Ok(
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
    #[test_case("rows from ( foo() ) with ordinality as s(y)" => Ok(
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

    #[test_case("foo()" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
    ))]
    #[test_case("bar() with ordinality" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["bar".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
        .with_ordinality(true)
    ))]
    #[test_case("baz() as t(x)" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["baz".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
        .with_alias(Both(
            "t".into(),
            vec![FuncAliasColumn::new("x")]
        ))
    ))]
    #[test_case("qux() with ordinality as s(y)" => Ok(
        FunctionTableRef::new(FuncCall::new(
            vec!["qux".into()],
            FuncArgsKind::Empty { order_within_group: None }
        ))
        .with_ordinality(true)
        .with_alias(Both(
            "s".into(),
            vec![FuncAliasColumn::new("y")]
        ))
    ))]
    fn test_function_stmt(source: &str) -> scan::Result<FunctionTableRef> {
        test_parser!(source, function_stmt)
    }

    #[test_case("xmltable('foo' passing 'bar' columns qux int)" => Ok(
        XmlTable::new(
            StringConst("bar".into()),
            StringConst("foo".into()),
            vec![XmlTableColumn::new("qux",
                XmlTableColumnDefinition::from(Int4)
            )]
        ).into()
    ))]
    #[test_case("json_table('foo', 'bar' columns(qux for ordinality))" => Ok(
        JsonTable::new(
            JsonValueExpr::from(StringConst("foo".into())),
            JsonTablePathSpec::new("bar"),
            vec![
                JsonTableColumnDefinition::ForOrdinality { column_name: "qux".into() }
            ]
        ).into()
    ))]
    #[test_case("rows from ( foo(1), bar(*) )" => matches Ok(_))]
    #[test_case("baz()" => matches Ok(_))]
    fn test_lateral_func_table(source: &str) -> scan::Result<TableRef> {
        test_parser!(source, lateral_func_table)
    }
}

use crate::combinators::core::Combinator;
use crate::combinators::func_expr_windowless;
use crate::combinators::table_ref::func_alias_clause;
use crate::combinators::table_ref::json_table;
use crate::combinators::table_ref::rowsfrom_list;
use crate::combinators::table_ref::xmltable;
use crate::paren;
use crate::seq;
use crate::ParserContext;
use pg_ast::FunctionTableRef;
use pg_ast::RowsTableRef;
use pg_ast::TableRef;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::FromKw;
use pg_lexer::Keyword::Ordinality;
use pg_lexer::Keyword::Rows;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Xmltable;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
