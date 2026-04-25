enum Change {
    Schema(Str),
    Options(Option<Vec<Str>>),
    Contents {
        action: AddDrop,
        target: AlterExtensionContentsTarget,
    }
}

/// Aliases:
/// * `AlterExtensionContentsStmt`
/// * `AlterExtensionStmt`
pub(in crate::combinators::stmt) fn alter_extension_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        ALTER EXTENSION ColId (
              SET SCHEMA ColId                      => AlterObjectSchemaStmt
            | UPDATE alter_extension_opt_list       => AlterExtensionStmt
            | ( ADD | DROP ) alter_extension_target => AlterExtensionContentsStmt
        )
    */

    let (_, extension, change) = seq!(
        Kw::Extension,
        col_id,
        alt!(
            change_schema,
            update_options,
            change_content
        )
    ).parse(ctx)?;

    let stmt = match change {
        Change::Schema(new_schema) => {
            AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Extension(extension),
                new_schema
            ).into()
        },
        Change::Options(options) => {
            let mut stmt = AlterExtensionStmt::new(extension);
            stmt.set_options(options);
            stmt.into()
        },
        Change::Contents { action, target } => {
            AlterExtensionContentsStmt::new(extension, action, target).into()
        },
    };

    Ok(stmt)
}

fn change_schema(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (.., new_schema) = seq!(Kw::Set, Kw::Schema, col_id)
        .parse(ctx)?;

    Ok(Change::Schema(new_schema))
}

fn update_options(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (_, options) = seq!(
        Kw::Update,
        alter_extension_options.optional()
    ).parse(ctx)?;

    Ok(Change::Options(options))
}

fn change_content(ctx: &mut ParserContext) -> scan::Result<Change> {

    let (action, target) = seq!(
        add_drop,
        alter_extension_target
    ).parse(ctx)?;

    Ok(Change::Contents { action, target })
}

/// Alias: `alter_extension_opt_list`
/// Includes: `alter_extension_opt_item`
fn alter_extension_options(ctx: &mut ParserContext) -> scan::Result<Vec<Str>> {

    /*
        ( TO NonReservedWord_or_Sconst )*
    */

    let options = many!(
        seq!(To, non_reserved_word_or_sconst)
            .map(|(_, opt)| opt)
    ).parse(ctx)?;

    Ok(options)
}

fn alter_extension_target(ctx: &mut ParserContext) -> scan::Result<AlterExtensionContentsTarget> {

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
        | ( PROCEDURAL )? LANGUAGE ColId
        | PROCEDURE function_with_argtypes
        | PROPERTY GRAPH any_name
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

    alt!(
        access_method.map(AccessMethod),
        aggregate.map(Aggregate),
        typecast.map(Typecast),
        collation.map(Collation),
        conversion.map(Conversion),
        database.map(Database),
        domain.map(Domain),
        event_trigger.map(EventTrigger),
        extension.map(Extension),
        foreign.map(|foreign| match foreign {
            Foreign::DataWrapper(foreign) => ForeignDataWrapper(foreign),
            Foreign::Table(foreign) => ForeignTable(foreign),
        }),
        function.map(Function),
        index.map(Index),
        materialized_view.map(MaterializedView),
        operator.map(|op| match op {
            Op::WithArgs(op) => Operator(op),
            Op::Class { name, index_method } => OperatorClass { name, index_method },
            Op::Family { name, index_method } => OperatorFamily { name, index_method },
        }),
        language.map(Language),
        procedure.map(Procedure),
        property_graph.map(PropertyGraph),
        publication.map(Publication),
        role.map(Role),
        routine.map(Routine),
        schema.map(Schema),
        sequence.map(Sequence),
        server.map(ForeignServer),
        statistics.map(ExtendedStatistics),
        subscription.map(Subscription),
        table.map(Table),
        tablespace.map(Tablespace),
        text_search.map(|text_search| match text_search {
            TextSearch::Configuration(name) => TextSearchConfiguration(name),
            TextSearch::Dictionary(name) => TextSearchDictionary(name),
            TextSearch::Parser(name) => TextSearchParser(name),
            TextSearch::Template(name) => TextSearchTemplate(name),
        }),
        transform.map(Transform),
        type_name.map(Type),
        view.map(View),
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::AggregateWithArgs;
    use pg_ast::FuncArgs::NoArgs;
    use pg_ast::FunctionWithArgs;
    use pg_ast::Transform as TransformAst;
    use pg_ast::TypeName::Int4;
    use pg_ast::TypeName::Varchar;
    use pg_ast::Typecast as Cast;
    use test_case::test_matrix;

    #[test_matrix("extension some_extension set schema some_schema" => Ok(
        AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Extension("some_extension".into()),
            "some_schema"
        ).into()
    ))]
    #[test_matrix("extension some_extension update to 'option1'" => Ok(
        AlterExtensionStmt::new("some_extension")
            .with_options(vec!["option1".into()])
            .into()
    ))]
    #[test_matrix("extension some_extension add aggregate some_aggregate(*)" => Ok(
        AlterExtensionContentsStmt::new(
            "some_extension",
            AddDrop::Add,
            Aggregate(AggregateWithArgs::new(
                vec!["some_aggregate".into()],
                vec![],
                vec![]
            ))
        ).into()
    ))]
    #[test_matrix("extension some_extension drop function some_function" => Ok(
        AlterExtensionContentsStmt::new(
            "some_extension",
            AddDrop::Drop,
            Function(FunctionWithArgs::new(vec!["some_function".into()], NoArgs))
        ).into()
    ))]
    fn test_alter_extension_stmt(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, alter_extension_stmt)
    }

    #[test]
    fn test_alter_extension_options() {
        test_parser!(
            source = r#"to "ident" to 'string' to reassign to trim to natural"#,
            parser = alter_extension_options,
            expected = vec![
                "ident".into(),
                "string".into(),
                "reassign".into(),
                "trim".into(),
                "natural".into()
            ]
        );
    }

    #[test_matrix("access method some_method" => Ok(AccessMethod("some_method".into())))]
    #[test_matrix("aggregate some_aggregate(*)" => Ok(
        Aggregate(AggregateWithArgs::new(
            vec!["some_aggregate".into()],
            vec![],
            vec![]
        ))
    ))]
    #[test_matrix("cast (int as varchar)" => Ok(
        Typecast(Cast::new(
            Int4,
            Varchar { max_length: None }
        ))
    ))]
    #[test_matrix("collation some_collation" => Ok(Collation(vec!["some_collation".into()])))]
    #[test_matrix("conversion some_conversion" => Ok(Conversion(vec!["some_conversion".into()])))]
    #[test_matrix("database some_database" => Ok(Database("some_database".into())))]
    #[test_matrix("domain int" => Ok(Domain(Int4.into())))]
    #[test_matrix("event trigger some_trigger" => Ok(EventTrigger("some_trigger".into())))]
    #[test_matrix("extension some_extension" => Ok(Extension("some_extension".into())))]
    #[test_matrix("foreign data wrapper some_wrapper" => Ok(ForeignDataWrapper("some_wrapper".into())))]
    #[test_matrix("foreign table some_table" => Ok(ForeignTable(vec!["some_table".into()])))]
    #[test_matrix("function some_function" => Ok(Function(
        FunctionWithArgs::new(vec!["some_function".into()], NoArgs)
    )))]
    #[test_matrix("index some_index" => Ok(Index(vec!["some_index".into()])))]
    #[test_matrix("materialized view some_view" => Ok(MaterializedView(vec!["some_view".into()])))]
    #[test_matrix("procedural language some_language" => Ok(Language("some_language".into())))]
    #[test_matrix("language some_language" => Ok(Language("some_language".into())))]
    #[test_matrix("procedure some_procedure" => Ok(Procedure(
        FunctionWithArgs::new(vec!["some_procedure".into()], NoArgs)
    )))]
    #[test_matrix("publication some_publication" => Ok(Publication("some_publication".into())))]
    #[test_matrix("property graph some_prop_graph" => Ok(PropertyGraph(vec!["some_prop_graph".into()])))]
    #[test_matrix("role some_role" => Ok(Role("some_role".into())))]
    #[test_matrix("routine some_routine" => Ok(Routine(
        FunctionWithArgs::new(vec!["some_routine".into()], NoArgs)
    )))]
    #[test_matrix("schema some_schema" => Ok(Schema("some_schema".into())))]
    #[test_matrix("sequence some_sequence" => Ok(Sequence(vec!["some_sequence".into()])))]
    #[test_matrix("server some_server" => Ok(ForeignServer("some_server".into())))]
    #[test_matrix("statistics some_statistics" => Ok(ExtendedStatistics(vec!["some_statistics".into()])))]
    #[test_matrix("subscription some_subscription" => Ok(Subscription("some_subscription".into())))]
    #[test_matrix("table some_table" => Ok(Table(vec!["some_table".into()])))]
    #[test_matrix("tablespace some_tablespace" => Ok(Tablespace("some_tablespace".into())))]
    #[test_matrix("text search configuration some_configuration" => Ok(
        TextSearchConfiguration(vec!["some_configuration".into()])
    ))]
    #[test_matrix("text search dictionary some_dictionary" => Ok(TextSearchDictionary(vec!["some_dictionary".into()])))]
    #[test_matrix("text search parser some_parser" => Ok(TextSearchParser(vec!["some_parser".into()])))]
    #[test_matrix("text search template some_template" => Ok(TextSearchTemplate(vec!["some_template".into()])))]
    #[test_matrix("transform for int language some_language" => Ok(Transform(TransformAst::new(Int4, "some_language"))))]
    #[test_matrix("type int" => Ok(Type(Int4.into())))]
    #[test_matrix("view some_view" => Ok(View(vec!["some_view".into()])))]
    fn test_alter_extension_target(source: &str) -> scan::Result<AlterExtensionContentsTarget> {
        test_parser!(source, alter_extension_target)
    }
}

use crate::alt;
use crate::combinators::add_drop;
use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::combinators::non_reserved_word_or_sconst;
use crate::combinators::stmt::access_method;
use crate::combinators::stmt::aggregate;
use crate::combinators::stmt::collation;
use crate::combinators::stmt::conversion;
use crate::combinators::stmt::database;
use crate::combinators::stmt::domain;
use crate::combinators::stmt::event_trigger;
use crate::combinators::stmt::extension;
use crate::combinators::stmt::foreign;
use crate::combinators::stmt::function;
use crate::combinators::stmt::index;
use crate::combinators::stmt::language;
use crate::combinators::stmt::materialized_view;
use crate::combinators::stmt::operator;
use crate::combinators::stmt::procedure;
use crate::combinators::stmt::property_graph;
use crate::combinators::stmt::publication;
use crate::combinators::stmt::role;
use crate::combinators::stmt::routine;
use crate::combinators::stmt::schema;
use crate::combinators::stmt::sequence;
use crate::combinators::stmt::server;
use crate::combinators::stmt::statistics;
use crate::combinators::stmt::subscription;
use crate::combinators::stmt::table;
use crate::combinators::stmt::tablespace;
use crate::combinators::stmt::text_search;
use crate::combinators::stmt::transform;
use crate::combinators::stmt::type_name;
use crate::combinators::stmt::typecast;
use crate::combinators::stmt::view;
use crate::combinators::stmt::Foreign;
use crate::combinators::stmt::Operator as Op;
use crate::combinators::stmt::TextSearch;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::AddDrop;
use pg_ast::AlterExtensionContentsStmt;
use pg_ast::AlterExtensionContentsTarget;
use pg_ast::AlterExtensionContentsTarget::AccessMethod;
use pg_ast::AlterExtensionContentsTarget::Aggregate;
use pg_ast::AlterExtensionContentsTarget::Collation;
use pg_ast::AlterExtensionContentsTarget::Conversion;
use pg_ast::AlterExtensionContentsTarget::Database;
use pg_ast::AlterExtensionContentsTarget::Domain;
use pg_ast::AlterExtensionContentsTarget::EventTrigger;
use pg_ast::AlterExtensionContentsTarget::ExtendedStatistics;
use pg_ast::AlterExtensionContentsTarget::Extension;
use pg_ast::AlterExtensionContentsTarget::ForeignDataWrapper;
use pg_ast::AlterExtensionContentsTarget::ForeignServer;
use pg_ast::AlterExtensionContentsTarget::ForeignTable;
use pg_ast::AlterExtensionContentsTarget::Function;
use pg_ast::AlterExtensionContentsTarget::Index;
use pg_ast::AlterExtensionContentsTarget::Language;
use pg_ast::AlterExtensionContentsTarget::MaterializedView;
use pg_ast::AlterExtensionContentsTarget::Operator;
use pg_ast::AlterExtensionContentsTarget::OperatorClass;
use pg_ast::AlterExtensionContentsTarget::OperatorFamily;
use pg_ast::AlterExtensionContentsTarget::Procedure;
use pg_ast::AlterExtensionContentsTarget::PropertyGraph;
use pg_ast::AlterExtensionContentsTarget::Publication;
use pg_ast::AlterExtensionContentsTarget::Role;
use pg_ast::AlterExtensionContentsTarget::Routine;
use pg_ast::AlterExtensionContentsTarget::Schema;
use pg_ast::AlterExtensionContentsTarget::Sequence;
use pg_ast::AlterExtensionContentsTarget::Subscription;
use pg_ast::AlterExtensionContentsTarget::Table;
use pg_ast::AlterExtensionContentsTarget::Tablespace;
use pg_ast::AlterExtensionContentsTarget::TextSearchConfiguration;
use pg_ast::AlterExtensionContentsTarget::TextSearchDictionary;
use pg_ast::AlterExtensionContentsTarget::TextSearchParser;
use pg_ast::AlterExtensionContentsTarget::TextSearchTemplate;
use pg_ast::AlterExtensionContentsTarget::Transform;
use pg_ast::AlterExtensionContentsTarget::Type;
use pg_ast::AlterExtensionContentsTarget::Typecast;
use pg_ast::AlterExtensionContentsTarget::View;
use pg_ast::AlterExtensionStmt;
use pg_ast::AlterObjectSchemaStmt;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::RawStmt;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
