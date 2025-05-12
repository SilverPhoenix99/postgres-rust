/// Aliases:
/// * `AlterExtensionContentsStmt`
/// * `AlterExtensionStmt`
pub(super) fn alter_extension_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER EXTENSION ColId (
              SET SCHEMA ColId                      => AlterObjectSchemaStmt
            | UPDATE alter_extension_opt_list       => AlterExtensionStmt
            | ( ADD | DROP ) alter_extension_target => AlterExtensionContentsStmt
        )
    */

    Kw::Extension.and_right(col_id())
        .chain(match_first_with_state!{|extension, stream| {
            {
                Kw::Set.and(Kw::Schema)
                    .and_right(col_id())
            } => (schema) {
                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Extension(extension),
                    schema
                ).into()
            },
            {
                Kw::Update
                    .and_right(alter_extension_options())
            } => (options) {
                AlterExtensionStmt::new(extension,options).into()
            },
            {
                and(
                    or(
                        Add.map(|_| AddDrop::Add),
                        DropKw.map(|_| AddDrop::Drop),
                    ),
                    alter_extension_target()
                )
            } => ((action, target)) {
                AlterExtensionContentsStmt::new(extension, action, target).into()
            }
        }})
}

/// Alias: `alter_extension_opt_list`
/// Includes: `alter_extension_opt_item`
///
/// Post-condition: Vec **May** be empty.
fn alter_extension_options() -> impl Combinator<Output = Vec<Str>> {

    /*
        ( TO NonReservedWord_or_Sconst )*
    */

    many(To.and_right(non_reserved_word_or_sconst()))
        .optional()
        .map(Option::unwrap_or_default)
}

fn alter_extension_target() -> impl Combinator<Output = AlterExtensionContentsTarget> {

    /*
          ACCESS METHOD ColId
        | AGGREGATE aggregate_with_argtypes
        | CAST '(' Typename AS Typename ')'
        | COLLATION any_name
        | CONVERSION_P any_name
        | DATABASE ColId
        | DOMAIN_P Typename
        | EVENT TRIGGER ColId
        | EXTENSION ColId
        | FOREIGN DATA_P WRAPPER ColId
        | FOREIGN TABLE any_name
        | FUNCTION function_with_argtypes
        | INDEX any_name
        | MATERIALIZED VIEW any_name
        | OPERATOR CLASS any_name USING ColId
        | OPERATOR FAMILY any_name USING ColId
        | OPERATOR operator_with_argtypes
        | opt_procedural LANGUAGE ColId
        | PROCEDURE function_with_argtypes
        | PUBLICATION ColId
        | ROLE ColId
        | ROUTINE function_with_argtypes
        | SCHEMA ColId
        | SEQUENCE any_name
        | SERVER ColId
        | STATISTICS any_name
        | SUBSCRIPTION ColId
        | TABLE any_name
        | TABLESPACE ColId
        | TEXT_P SEARCH CONFIGURATION any_name
        | TEXT_P SEARCH DICTIONARY any_name
        | TEXT_P SEARCH PARSER any_name
        | TEXT_P SEARCH TEMPLATE any_name
        | TRANSFORM FOR Typename LANGUAGE ColId
        | TYPE_P Typename
        | VIEW any_name
    */

    match_first! {
        access_method().map(AccessMethod),
        aggregate().map(Aggregate),
        typecast().map(Typecast),
        collation().map(Collation),
        conversion().map(Conversion),
        database().map(Database),
        domain().map(Domain),
        event_trigger().map(EventTrigger),
        extension().map(Extension),
        foreign().map(|foreign| match foreign {
            Foreign::DataWrapper(foreign) => ForeignDataWrapper(foreign),
            Foreign::Table(foreign) => ForeignTable(foreign),
        }),
        function().map(Function),
        index().map(Index),
        materialized_view().map(MaterializedView),
        operator().map(|op| match op {
            Op::WithArgs(op) => Operator(op),
            Op::Class { name, index_method } => OperatorClass { name, index_method },
            Op::Family { name, index_method } => OperatorFamily { name, index_method },
        }),
        language().map(Language),
        procedure().map(Procedure),
        publication().map(Publication),
        role().map(Role),
        routine().map(Routine),
        schema().map(Schema),
        sequence().map(Sequence),
        server().map(ForeignServer),
        statistics().map(ExtendedStatistics),
        subscription().map(Subscription),
        table().map(Table),
        tablespace().map(Tablespace),
        text_search().map(|text_search| match text_search {
            TextSearch::Configuration(name) => TextSearchConfiguration(name),
            TextSearch::Dictionary(name) => TextSearchDictionary(name),
            TextSearch::Parser(name) => TextSearchParser(name),
            TextSearch::Template(name) => TextSearchTemplate(name),
        }),
        transform().map(Transform),
        type_name().map(Type),
        view().map(View),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::{
        AggregateWithArgs,
        FunctionWithArgs,
        Transform as TransformAst,
        TypeName::{Int4, Varchar},
        Typecast as Cast,
    };
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("extension some_extension set schema some_schema",
        AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Extension("some_extension".into()),
            "some_schema"
        ).into()
    )]
    #[test_case("extension some_extension update to 'option1'",
        AlterExtensionStmt::new(
            "some_extension",
            vec!["option1".into()]
        ).into()
    )]
    #[test_case("extension some_extension add aggregate some_aggregate(*)",
        AlterExtensionContentsStmt::new(
            "some_extension",
            AddDrop::Add,
            Aggregate(AggregateWithArgs::new(
                vec!["some_aggregate".into()],
                vec![],
                vec![]
            ))
        ).into()
    )]
    #[test_case("extension some_extension drop function some_function",
        AlterExtensionContentsStmt::new(
            "some_extension",
            AddDrop::Drop,
            Function(FunctionWithArgs::new(vec!["some_function".into()], None))
        ).into()
    )]
    fn test_alter_extension_stmt(source: &str, expected: RawStmt) {
        test_parser!(source, alter_extension_stmt(), expected);
    }

    #[test]
    fn test_alter_extension_options() {
        test_parser!(
            source = r#"to "ident" to 'string' to reassign to trim to natural"#,
            parser = alter_extension_options(),
            expected = vec![
                "ident".into(),
                "string".into(),
                "reassign".into(),
                "trim".into(),
                "natural".into()
            ]
        );
    }

    #[test_case("access method some_method", AccessMethod("some_method".into()))]
    #[test_case("aggregate some_aggregate(*)",
        Aggregate(AggregateWithArgs::new(
            vec!["some_aggregate".into()],
            vec![],
            vec![]
        ))
    )]
    #[test_case("cast (int as varchar)",
        Typecast(Cast::new(
            Int4,
            Varchar { max_length: None }
        ))
    )]
    #[test_case("collation some_collation", Collation(vec!["some_collation".into()]))]
    #[test_case("conversion some_conversion", Conversion(vec!["some_conversion".into()]))]
    #[test_case("database some_database", Database("some_database".into()))]
    #[test_case("domain int", Domain(Int4.into()))]
    #[test_case("event trigger some_trigger", EventTrigger("some_trigger".into()))]
    #[test_case("extension some_extension", Extension("some_extension".into()))]
    #[test_case("foreign data wrapper some_wrapper", ForeignDataWrapper("some_wrapper".into()))]
    #[test_case("foreign table some_table", ForeignTable(vec!["some_table".into()]))]
    #[test_case("function some_function", Function(
        FunctionWithArgs::new(vec!["some_function".into()], None)
    ))]
    #[test_case("index some_index", Index(vec!["some_index".into()]))]
    #[test_case("materialized view some_view", MaterializedView(vec!["some_view".into()]))]
    #[test_case("procedural language some_language", Language("some_language".into()))]
    #[test_case("language some_language", Language("some_language".into()))]
    #[test_case("procedure some_procedure", Procedure(
        FunctionWithArgs::new(vec!["some_procedure".into()], None)
    ))]
    #[test_case("publication some_publication", Publication("some_publication".into()))]
    #[test_case("role some_role", Role("some_role".into()))]
    #[test_case("routine some_routine", Routine(
        FunctionWithArgs::new(vec!["some_routine".into()], None)
    ))]
    #[test_case("schema some_schema", Schema("some_schema".into()))]
    #[test_case("sequence some_sequence", Sequence(vec!["some_sequence".into()]))]
    #[test_case("server some_server", ForeignServer("some_server".into()))]
    #[test_case("statistics some_statistics", ExtendedStatistics(vec!["some_statistics".into()]))]
    #[test_case("subscription some_subscription", Subscription("some_subscription".into()))]
    #[test_case("table some_table", Table(vec!["some_table".into()]))]
    #[test_case("tablespace some_tablespace", Tablespace("some_tablespace".into()))]
    #[test_case("text search configuration some_configuration",
        TextSearchConfiguration(vec!["some_configuration".into()])
    )]
    #[test_case("text search dictionary some_dictionary", TextSearchDictionary(vec!["some_dictionary".into()]))]
    #[test_case("text search parser some_parser", TextSearchParser(vec!["some_parser".into()]))]
    #[test_case("text search template some_template", TextSearchTemplate(vec!["some_template".into()]))]
    #[test_case("transform for int language some_language", Transform(TransformAst::new(Int4, "some_language")))]
    #[test_case("type int", Type(Int4.into()))]
    #[test_case("view some_view", View(vec!["some_view".into()]))]
    fn test_alter_extension_target(source: &str, expected: AlterExtensionContentsTarget) {
        test_parser!(source, alter_extension_target(), expected);
    }
}

use crate::parser::ast_node::AddDrop;
use crate::parser::ast_node::AlterExtensionContentsStmt;
use crate::parser::ast_node::AlterExtensionContentsTarget;
use crate::parser::ast_node::AlterExtensionContentsTarget::*;
use crate::parser::ast_node::AlterExtensionStmt;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterObjectSchemaTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::non_reserved_word_or_sconst;
use crate::parser::combinators::stmt::access_method;
use crate::parser::combinators::stmt::aggregate;
use crate::parser::combinators::stmt::collation;
use crate::parser::combinators::stmt::conversion;
use crate::parser::combinators::stmt::database;
use crate::parser::combinators::stmt::domain;
use crate::parser::combinators::stmt::event_trigger;
use crate::parser::combinators::stmt::extension;
use crate::parser::combinators::stmt::foreign;
use crate::parser::combinators::stmt::function;
use crate::parser::combinators::stmt::index;
use crate::parser::combinators::stmt::language;
use crate::parser::combinators::stmt::materialized_view;
use crate::parser::combinators::stmt::operator;
use crate::parser::combinators::stmt::procedure;
use crate::parser::combinators::stmt::publication;
use crate::parser::combinators::stmt::role;
use crate::parser::combinators::stmt::routine;
use crate::parser::combinators::stmt::schema;
use crate::parser::combinators::stmt::sequence;
use crate::parser::combinators::stmt::server;
use crate::parser::combinators::stmt::statistics;
use crate::parser::combinators::stmt::subscription;
use crate::parser::combinators::stmt::table;
use crate::parser::combinators::stmt::tablespace;
use crate::parser::combinators::stmt::text_search;
use crate::parser::combinators::stmt::transform;
use crate::parser::combinators::stmt::type_name;
use crate::parser::combinators::stmt::typecast;
use crate::parser::combinators::stmt::view;
use crate::parser::combinators::stmt::Foreign;
use crate::parser::combinators::stmt::Operator as Op;
use crate::parser::combinators::stmt::TextSearch;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::Add;
use postgres_parser_lexer::Keyword::DropKw;
use postgres_parser_lexer::Keyword::To;
