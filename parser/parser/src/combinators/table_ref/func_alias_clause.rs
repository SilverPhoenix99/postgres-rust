pub(super) fn func_alias_clause(stream: &mut TokenStream) -> scan::Result<FuncAlias> {

    /*
          AS '(' TableFuncElementList ')'
        | AS named_alias
        |    named_alias
    */

    alt!(
        seq!(As, alt!(
            paren!(table_func_element_list)
                .map(|cols| {
                    let cols = cols.into_iter()
                        .map(FuncAliasColumn::from)
                        .collect();
                    FuncAlias::Right(cols)
                }),
            named_alias
        )).map(|(_, alias)| alias),
        named_alias
    ).parse(stream)
}

fn named_alias(stream: &mut TokenStream) -> scan::Result<FuncAlias> {

    /*
        col_id ( func_alias_columns )?
    */

    let (alias, cols) = seq!(col_id, func_alias_columns.optional())
        .parse(stream)?;

    let alias = match cols {
        None => FuncAlias::Left(alias),
        Some(cols) => FuncAlias::Both(alias, cols),
    };

    Ok(alias)
}

fn func_alias_columns(stream: &mut TokenStream) -> scan::Result<Vec<FuncAliasColumn>> {

    /*
        '(' func_alias_column_list ')'
    */

    paren!(func_alias_column_list)
        .parse(stream)
}

fn func_alias_column_list(stream: &mut TokenStream) -> scan::Result<Vec<FuncAliasColumn>> {

    /*
        func_alias_column ( ',' ( name_list | TableFuncElementList ) )?

        What production comes after, depends on the 1st column.
        It guarantees all columns have a type, or none of them do, as enforced by C-PG.
    */

    let first = func_alias_column(stream)?;

    let mut columns: Vec<_> = match first.type_name() {
        None => {
            // ( ',' name_list )?
            seq!(Comma, name_list)
                .parse(stream)
                .optional()?
                .into_iter()
                .flat_map(|(_, names)| names)
                .map(FuncAliasColumn::new)
                .collect()
        }
        Some(_) => {
            // ( ',' TableFuncElementList )?
            seq!(Comma, table_func_element_list)
                .parse(stream)
                .optional()?
                .into_iter()
                .flat_map(|(_, elements)| elements)
                .map(FuncAliasColumn::from)
                .collect()
        }
    };

    columns.insert(0, first);
    Ok(columns)
}

fn func_alias_column(stream: &mut TokenStream) -> scan::Result<FuncAliasColumn> {

    /*
    Technically:
        col_id | TableFuncElement

    In practice (with validation):
        col_id ( typename ( collate_clause )? )?
    */

    let (alias, tail) = seq!(
        col_id,
        seq!(
            typename,
            collate_clause.optional()
        ).optional()
    ).parse(stream)?;

    let (type_name, collation) = match tail {
        Some((type_name, collation)) => (Some(type_name), collation),
        None => (None, None),
    };

    let mut alias = FuncAliasColumn::new(alias);
    alias.set_type_name(type_name)
        .set_collation(collation);

    Ok(alias)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::TypeName::{Generic, Int4},
        pg_basics::Location,
        pg_elog::parser::Error::Syntax,
    };

    #[test_case("as (x int)" => Ok(
        FuncAlias::Right(vec![
            FuncAliasColumn::new("x")
                .with_type_name(Int4)
        ])
    ))]
    #[test_case("as foo(x)" => matches Ok(_))]
    #[test_case("bar(x int)" => matches Ok(_))]
    fn test_func_alias_clause(source: &str) -> scan::Result<FuncAlias> {
        test_parser!(source, func_alias_clause)
    }

    #[test_case("narslog" => Ok(
        FuncAlias::Left("narslog".into())
    ))]
    #[test_case("umpus(x)" => Ok(
        FuncAlias::Both(
            "umpus".into(),
            vec![FuncAliasColumn::new("x")]
        )
    ))]
    #[test_case("wawas(x int)" => Ok(
        FuncAlias::Both(
            "wawas".into(),
            vec![
                FuncAliasColumn::new("x")
                    .with_type_name(Int4)
            ]
        )
    ))]
    fn test_named_alias(source: &str) -> scan::Result<FuncAlias> {
        test_parser!(source, named_alias)
    }

    #[test_case("(foo, bar)" => matches Ok(_))]
    #[test_case("(baz int, qux int)" => matches Ok(_))]
    #[test_case("(umpus int, narslog)" => Err(
        Syntax
            .at(Location::new(19..20, 1, 20)) // ')'
            .into()
    ))]
    #[test_case("(wawas, narslog int)" => Err(
        Syntax
            .at(Location::new(16..19, 1, 17)) // "int"
            .into()
    ))]
    fn test_func_alias_columns(source: &str) -> scan::Result<Vec<FuncAliasColumn>> {
        test_parser!(source, func_alias_columns)
    }

    #[test_case("foo, bar" => Ok(vec![
        FuncAliasColumn::new("foo"),
        FuncAliasColumn::new("bar"),
    ]))]
    #[test_case("baz int collate lorem, qux int" => Ok(vec![
        FuncAliasColumn::new("baz")
            .with_type_name(Int4)
            .with_collation(vec!["lorem".into()]),
        FuncAliasColumn::new("qux")
            .with_type_name(Int4)
    ]))]
    fn test_func_alias_column_list(source: &str) -> scan::Result<Vec<FuncAliasColumn>> {
        test_parser!(source, func_alias_column_list)
    }

    #[test_case("foo" => Ok(
        FuncAliasColumn::new("foo")
    ))]
    #[test_case("bar int" => Ok(
        FuncAliasColumn::new("bar")
            .with_type_name(Int4)
    ))]
    #[test_case("qux text collate lorem" => Ok(
        FuncAliasColumn::new("qux")
            .with_type_name(
                Generic {
                    name: vec!["text".into()],
                    type_modifiers: None
                }
            )
            .with_collation(vec!["lorem".into()])
    ))]
    fn test_func_alias_column(source: &str) -> scan::Result<FuncAliasColumn> {
        test_parser!(source, func_alias_column)
    }
}

use crate::combinators::col_id;
use crate::combinators::collate_clause;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::name_list;
use crate::combinators::table_func_element_list;
use crate::combinators::typename;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::FuncAlias;
use pg_ast::FuncAliasColumn;
use pg_lexer::Keyword::As;
use pg_lexer::OperatorKind::Comma;
