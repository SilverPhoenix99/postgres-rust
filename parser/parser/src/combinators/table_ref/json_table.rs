fn json_table_column_definition_list(stream: &mut TokenStream) -> scan::Result<Vec<JsonTableColumnDefinition>> {

    /*
        json_table_column_definition ( ',' json_table_column_definition )*
    */

    many_sep(Comma, json_table_column_definition)
        .parse(stream)
}

enum PartialColumnDefinition {
    ForOrdinality,
    Other {
        type_name: Type,
        tail: ColumnDefinitionTail,
    }
}

enum ColumnDefinitionTail {
    Exists(ExistsColumnTail),
    Regular(RegularColumnTail),
}

struct ExistsColumnTail {
    path_spec: Option<JsonTablePathSpec>,
    on_error: Option<JsonBehavior>
}

struct RegularColumnTail {
    wrapper: JsonWrapperBehavior,
    format: Option<JsonFormat>,
    path_spec: Option<JsonTablePathSpec>,
    quotes: Option<JsonQuotes>,
    behavior: Option<JsonBehaviorClause>
}

impl_from!(ExistsColumnTail for ColumnDefinitionTail::Exists);
impl_from!(RegularColumnTail for ColumnDefinitionTail::Regular);

fn json_table_column_definition(stream: &mut TokenStream) -> scan::Result<JsonTableColumnDefinition> {

    /*
          NESTED ( PATH )? SCONST ( AS ColId )? COLUMNS '(' json_table_column_definition_list ')'
        | ColId FOR ORDINALITY
        | ColId Typename EXISTS ( json_table_column_path_clause )? ( json_on_error_clause )?
        | ColId Typename
            ( json_format_clause )?
            ( json_table_column_path_clause )?
            json_wrapper_behavior
            ( json_quotes_clause )?
            ( json_behavior_clause )?
    */

    alt!(
        nested_json_column.map(From::from),
        json_column
    ).parse(stream)
}

fn nested_json_column(stream: &mut TokenStream) -> scan::Result<JsonTableNestedColumn> {

    /*
        NESTED ( PATH )? SCONST ( AS ColId )? COLUMNS '(' json_table_column_definition_list ')'
    */

    let (_, _, path_spec, alias, _, columns) = seq!(
        Kw::Nested,
        Path.optional(),
        string,
        alias.optional(),
        Columns,
        paren(json_table_column_definition_list)
    ).parse(stream)?;

    let mut path_spec = JsonTablePathSpec::new(path_spec);
    path_spec.set_name(alias);

    let nested_column = JsonTableNestedColumn::new(path_spec, columns);

    Ok(nested_column)
}

fn json_column(stream: &mut TokenStream) -> scan::Result<JsonTableColumnDefinition> {

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
    )).parse(stream)?;

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

fn exists_column_tail(stream: &mut TokenStream) -> scan::Result<ExistsColumnTail> {

    /*
        EXISTS ( json_table_column_path_clause )? ( json_on_error_clause )?
    */

    let (_, path_spec, on_error) = seq!(
        Kw::Exists,
        json_table_column_path_clause.optional(),
        json_on_error_clause.optional()
    ).parse(stream)?;

    Ok(ExistsColumnTail {
        path_spec,
        on_error
    })
}

fn regular_column_tail(stream: &mut TokenStream) -> scan::Result<RegularColumnTail> {

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
    ).parse(stream)?;

    Ok(RegularColumnTail {
        wrapper,
        format,
        path_spec,
        quotes,
        behavior
    })
}

fn json_table_column_path_clause(stream: &mut TokenStream) -> scan::Result<JsonTablePathSpec> {

    /*
        PATH SCONST
    */

    let (_, path_spec) = seq!(Path, string)
        .parse(stream)?;

    let path_spec = JsonTablePathSpec::new(path_spec);
    Ok(path_spec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        JsonBehavior,
        JsonBehaviorClause,
        JsonFormat,
        JsonQuotes,
        JsonWrapperBehavior,
        TypeName::Int4,
    };
    use test_case::test_case;

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

use crate::combinators::col_id::col_id;
use crate::combinators::foundation::{paren, seq};
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::{alt, many_sep};
use crate::combinators::typename::typename;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::JsonTableColumnDefinition::ForOrdinality;
use pg_ast::JsonTableNestedColumn;
use pg_ast::JsonTablePathSpec;
use pg_ast::{JsonBehavior, JsonBehaviorClause, JsonFormat, JsonQuotes, JsonTableColumnDefinition, JsonWrapperBehavior, Type};
use pg_ast::{JsonTableExistsColumn, JsonTableRegularColumn};
use pg_basics::impl_from;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::{Columns, Ordinality, Path};
use pg_lexer::Keyword::For;
use pg_lexer::OperatorKind::Comma;
use crate::combinators::alias::alias;
use crate::combinators::json_behavior::{json_behavior_clause, json_on_error_clause};
use crate::combinators::json_format_clause::json_format_clause;
use crate::combinators::json_quotes_clause::json_quotes_clause;
use crate::combinators::json_wrapper_behavior::json_wrapper_behavior;
