pub(super) fn json_table(ctx: &mut ParserContext) -> scan::Result<JsonTable> {

    /*
        JSON_TABLE '('
            json_value_expr
            ','
            path_spec
            ( json_passing_clause )?
            json_table_column_definition_list
            ( json_on_error_clause )?
        ')'
    */

    let (_, (ctx, _, path_spec, passing, columns, on_error)) = seq!(Kw::JsonTable, paren!(seq!(
        json_value_expr,
        Comma,
        path_spec,
        json_passing_clause.optional(),
        json_table_column_definition_list,
        json_on_error_clause.optional()
    ))).parse(ctx)?;

    let mut expr = JsonTable::new(ctx, path_spec, columns);
    expr.set_passing(passing)
        .set_on_error(on_error);

    Ok(expr)
}

fn path_spec(ctx: &mut ParserContext) -> scan::Result<JsonTablePathSpec> {

    /*
        a_expr ( alias )?
    */

    let (Located(path, path_loc), alias) = seq!(
        located!(a_expr),
        alias.optional()
    ).parse(ctx)?;

    let StringConst(path) = path else {
        return Err(NonStringJsonTablePathSpec.at_location(path_loc).into());
    };

    let mut path_spec = JsonTablePathSpec::new(path);
    path_spec.set_name(alias);

    Ok(path_spec)
}

fn json_table_column_definition_list(ctx: &mut ParserContext) -> scan::Result<Vec<JsonTableColumnDefinition>> {

    /*
        COLUMNS '('
            json_table_column_definition ( ',' json_table_column_definition )*
        ')'
    */

    let (_, columns) = seq!(
        Columns,
        paren!(
            many!(sep = Comma, json_table_column_definition)
        )
    ).parse(ctx)?;

    Ok(columns)
}

enum PartialColumnDefinition {
    ForOrdinality,
    Other {
        type_name: Type,
        tail: ColumnDefinitionTail,
    },
}

#[derive(From)]
enum ColumnDefinitionTail {
    Exists(ExistsColumnTail),
    Regular(RegularColumnTail),
}

struct ExistsColumnTail {
    path_spec: Option<JsonTablePathSpec>,
    on_error: Option<JsonBehavior>,
}

struct RegularColumnTail {
    wrapper: JsonWrapperBehavior,
    format: Option<JsonFormat>,
    path_spec: Option<JsonTablePathSpec>,
    quotes: Option<JsonQuotes>,
    behavior: Option<JsonBehaviorClause>,
}

fn json_table_column_definition(ctx: &mut ParserContext) -> scan::Result<JsonTableColumnDefinition> {

    alt!(
        nested_json_column.map(From::from),
        json_column
    ).parse(ctx)
}

fn nested_json_column(ctx: &mut ParserContext) -> scan::Result<JsonTableNestedColumn> {

    /*
        NESTED ( PATH )? SCONST ( AS ColId )? json_table_column_definition_list
    */

    let (_, _, path_spec, alias, columns) = seq!(
        Kw::Nested,
        Path.optional(),
        string,
        alias.optional(),
        json_table_column_definition_list
    ).parse(ctx)?;

    let mut path_spec = JsonTablePathSpec::new(path_spec);
    path_spec.set_name(alias);

    let nested_column = JsonTableNestedColumn::new(path_spec, columns);

    Ok(nested_column)
}

fn json_column(ctx: &mut ParserContext) -> scan::Result<JsonTableColumnDefinition> {

    let (column_name, partial) = seq!(col_id, alt!(
        seq!(For, Ordinality)
            .map(|_| PartialColumnDefinition::ForOrdinality),
        seq!(typename, alt!(
            exists_column_tail.map(From::from),
            regular_column_tail.map(From::from)
        ))
            .map(|(type_name, tail)|
                PartialColumnDefinition::Other { type_name, tail }
            )
    )).parse(ctx)?;

    let column = match partial {

        PartialColumnDefinition::ForOrdinality => ForOrdinality { column_name },

        PartialColumnDefinition::Other {
            type_name,
            tail: ColumnDefinitionTail::Exists(ExistsColumnTail {
                path_spec,
                on_error
            })
        }
        => {
            let mut col = JsonTableExistsColumn::new(column_name, type_name);
            col.set_path_spec(path_spec)
                .set_on_error(on_error);
            col.into()
        },

        PartialColumnDefinition::Other {
            type_name,
            tail: ColumnDefinitionTail::Regular(RegularColumnTail {
                wrapper,
                format,
                path_spec,
                quotes,
                behavior
            })
        } => {
            let mut col = JsonTableRegularColumn::new(column_name, type_name, wrapper);
            col.set_format(format)
                .set_path_spec(path_spec)
                .set_quotes(quotes)
                .set_behavior(behavior);
            col.into()
        }
    };

    Ok(column)
}

fn exists_column_tail(ctx: &mut ParserContext) -> scan::Result<ExistsColumnTail> {

    /*
        EXISTS ( json_table_column_path_clause )? ( json_on_error_clause )?
    */

    let (_, path_spec, on_error) = seq!(
        Kw::Exists,
        json_table_column_path_clause.optional(),
        json_on_error_clause.optional()
    ).parse(ctx)?;

    Ok(ExistsColumnTail {
        path_spec,
        on_error
    })
}

fn regular_column_tail(ctx: &mut ParserContext) -> scan::Result<RegularColumnTail> {

    /*
        ( json_format_clause )?
            ( json_table_column_path_clause )?
            json_wrapper_behavior
            ( json_quotes_clause )?
            ( json_behavior_clause )?
    */

    let (format, path_spec, wrapper, quotes, behavior) = seq!(
        json_format_clause.optional(),
        json_table_column_path_clause.optional(),
        json_wrapper_behavior,
        json_quotes_clause.optional(),
        json_behavior_clause.optional()
    ).parse(ctx)?;

    Ok(RegularColumnTail {
        wrapper,
        format,
        path_spec,
        quotes,
        behavior
    })
}

fn json_table_column_path_clause(ctx: &mut ParserContext) -> scan::Result<JsonTablePathSpec> {

    /*
        PATH SCONST
    */

    let (_, path_spec) = seq!(Path, string)
        .parse(ctx)?;

    let path_spec = JsonTablePathSpec::new(path_spec);
    Ok(path_spec)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::TypeName::Int4;
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::JsonValueExpr,
        pg_elog::Error::Parser,
        scan::Error::ScanErr,
    };

    #[test_case(
        "json_table(\
            'doc', \
            'path' \
            columns(\
               bar for ordinality\
            )\
        )"
        => Ok(
            JsonTable::new(
                JsonValueExpr::from(StringConst("doc".into())),
                JsonTablePathSpec::new("path"),
                vec![
                    ForOrdinality { column_name: "bar".into() }
                ]
            )
        )
    )]
    #[test_case(
        "json_table(\
            'umpus' format json, \
            'wawas' as foo \
            passing 'lorem' as ipsum \
            columns(\
               qux for ordinality\
            ) \
            null on error\
        )"
        => Ok(
            JsonTable::new(
                JsonValueExpr::new(
                    StringConst("umpus".into()),
                    JsonFormat::text()
                ),
                JsonTablePathSpec::new("wawas")
                    .with_name("foo"),
                vec![
                    ForOrdinality { column_name: "qux".into() }
                ]
            )
            .with_passing(vec![
                ("ipsum".into(), JsonValueExpr::from(StringConst("lorem".into())))
            ])
            .with_on_error(JsonBehavior::Null)
        )
    )]
    fn test_json_table(source: &str) -> scan::Result<JsonTable> {
        test_parser!(source, json_table)
    }

    #[test_case("'foo'" => Ok(JsonTablePathSpec::new("foo")))]
    #[test_case("'foo' as bar" => Ok(
        JsonTablePathSpec::new("foo")
            .with_name("bar")
    ))]
    #[test_case("1" => matches Err(ScanErr(
        Located(Parser(NonStringJsonTablePathSpec), _)
    )))]
    fn test_path_spec(source: &str) -> scan::Result<JsonTablePathSpec> {
        test_parser!(source, path_spec)
    }

    #[test_case("foo for ordinality" => Ok(
        ForOrdinality {
            column_name: "foo".into(),
        }
    ))]
    #[test_case("bar int exists" => Ok(
        JsonTableExistsColumn::new("bar", Int4).into()
    ))]
    #[test_case(
        "baz int exists \
            path 'baz/path' \
            false on error"
        => Ok(
            JsonTableExistsColumn::new("baz", Int4)
                .with_path_spec(JsonTablePathSpec::new("baz/path"))
                .with_on_error(JsonBehavior::False)
                .into()
        )
    )]
    #[test_case("qux int without wrapper" => Ok(
        JsonTableRegularColumn::new("qux", Int4, JsonWrapperBehavior::Without).into()
    ))]
    #[test_case(
        "yumyum int \
            format json \
            path 'yumyum/path' \
            with wrapper \
            keep quotes \
            error on empty"
        => Ok(
            JsonTableRegularColumn::new("yumyum", Int4, JsonWrapperBehavior::Unconditional)
                .with_format(JsonFormat::text())
                .with_path_spec(JsonTablePathSpec::new("yumyum/path"))
                .with_quotes(JsonQuotes::Keep)
                .with_behavior(
                    JsonBehaviorClause::new()
                        .with_on_empty(JsonBehavior::Error)
                )
                .into()
        )
    )]
    #[test_case(
        "nested 'narslog/nested' \
            columns(\
                umpus int exists\
            )"
        => Ok(
            JsonTableNestedColumn::new(
                JsonTablePathSpec::new("narslog/nested"),
                vec![
                    JsonTableExistsColumn::new("umpus", Int4).into()
                ]
            )
                .into()
        )
    )]
    #[test_case(
        "nested \
            path 'wawas/nested' as lorem \
            columns(\
                ipsum for ordinality\
            )"
        => Ok(
            JsonTableNestedColumn::new(
                JsonTablePathSpec::new("wawas/nested")
                    .with_name("lorem"),
                vec![
                    ForOrdinality { column_name: "ipsum".into() }
                ]
            )
                .into()
        )
    )]
    fn test_json_table_column_definition(source: &str) -> scan::Result<JsonTableColumnDefinition> {
        test_parser!(source, json_table_column_definition)
    }

    #[test_case("path 'foo'" => Ok(JsonTablePathSpec::new("foo")))]
    fn test_json_table_column_path_clause(source: &str) -> scan::Result<JsonTablePathSpec> {
        test_parser!(source, json_table_column_path_clause)
    }
}

use crate::combinators::alias;
use crate::combinators::expr::a_expr;
use crate::combinators::json_behavior_clause;
use crate::combinators::json_format_clause;
use crate::combinators::json_on_error_clause;
use crate::combinators::json_passing_clause;
use crate::combinators::json_quotes_clause;
use crate::combinators::json_value_expr;
use crate::combinators::json_wrapper_behavior;
use derive_more::From;
use pg_ast::ExprNode::StringConst;
use pg_ast::JsonBehavior;
use pg_ast::JsonBehaviorClause;
use pg_ast::JsonFormat;
use pg_ast::JsonQuotes;
use pg_ast::JsonTable;
use pg_ast::JsonTableColumnDefinition;
use pg_ast::JsonTableColumnDefinition::ForOrdinality;
use pg_ast::JsonTableExistsColumn;
use pg_ast::JsonTableNestedColumn;
use pg_ast::JsonTablePathSpec;
use pg_ast::JsonTableRegularColumn;
use pg_ast::JsonWrapperBehavior;
use pg_ast::Type;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_combinators::alt;
use pg_combinators::located;
use pg_combinators::many;
use pg_combinators::paren;
use pg_combinators::seq;
use pg_combinators::string;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_elog::parser::Error::NonStringJsonTablePathSpec;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Columns;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Ordinality;
use pg_lexer::Keyword::Path;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_sink_combinators::col_id;
use pg_type_combinators::typename;
