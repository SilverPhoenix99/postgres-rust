/// Alias: `SecLabelStmt`
pub(super) fn security_label_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        SECURITY LABEL ( provider )? ON label_target IS security_label
    */

    let (.., provider, _, target, label) = seq!(
        Security,
        Label,
        provider.optional(),
        On,
        label_target,
        security_label
    ).parse(ctx)?;

    let stmt = match target {
        Target::Database { db_name } => {
            let label = SecurityLabel::new(provider, label);
            let option = DatabaseStmtOption::SecurityLabel(label);
            DatabaseStmt::new(db_name, option).into()
        }
        Target::Label(target) => {
            let label = SecurityLabel::new(provider, label);
            SecurityLabelStmt::new(target, label).into()
        }
    };

    Ok(stmt)
}

/// Alias: `opt_provider`
fn provider(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        FOR NonReservedWord_or_Sconst
    */

    let (_, provider) = seq!(For, non_reserved_word_or_sconst).parse(ctx)?;

    Ok(provider)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Target {
    Database { db_name: Str },
    Label(SecurityLabelTarget),
}

fn label_target(ctx: &mut ParserContext) -> scan::Result<Target> {

    /*
        ACCESS METHOD name
      | AGGREGATE aggregate_with_argtypes
      | COLLATION any_name
      | COLUMN any_name
      | CONVERSION_P any_name
      | DATABASE name
      | DOMAIN_P Typename
      | EVENT TRIGGER name
      | EXTENSION name
      | FOREIGN DATA_P WRAPPER name
      | FOREIGN TABLE any_name
      | FUNCTION function_with_argtypes
      | INDEX any_name
      | LARGE_P OBJECT_P NumericOnly
      | MATERIALIZED VIEW any_name
      | ( PROCEDURAL )? LANGUAGE name
      | PROCEDURE function_with_argtypes
      | PROPERTY GRAPH any_name
      | PUBLICATION name
      | ROLE name
      | ROUTINE function_with_argtypes
      | SCHEMA name
      | SEQUENCE any_name
      | SERVER name
      | STATISTICS any_name
      | SUBSCRIPTION name
      | TABLE any_name
      | TABLESPACE name
      | TEXT_P SEARCH CONFIGURATION any_name
      | TEXT_P SEARCH DICTIONARY any_name
      | TEXT_P SEARCH PARSER any_name
      | TEXT_P SEARCH TEMPLATE any_name
      | TYPE_P Typename
      | VIEW any_name
    */

    alt!(
        access_method.map(AccessMethod).map(Target::Label),
        aggregate.map(Aggregate).map(Target::Label),
        collation.map(Collation).map(Target::Label),
        column.map(Column).map(Target::Label),
        conversion.map(Conversion).map(Target::Label),
        database.map(|db_name| Target::Database { db_name }),
        domain.map(Domain).map(Target::Label),
        event_trigger.map(EventTrigger).map(Target::Label),
        extension.map(Extension).map(Target::Label),
        foreign.map(|foreign| match foreign {
            Foreign::DataWrapper(name) => ForeignDataWrapper(name),
            Foreign::Table(name) => ForeignTable(name),
        }).map(Target::Label),
        function.map(Function).map(Target::Label),
        index.map(Index).map(Target::Label),
        large_object.map(LargeObject).map(Target::Label),
        materialized_view.map(MaterializedView).map(Target::Label),
        language.map(Language).map(Target::Label),
        procedure.map(Procedure).map(Target::Label),
        property_graph.map(PropertyGraph).map(Target::Label),
        publication.map(Publication).map(Target::Label),
        role.map(Role).map(Target::Label),
        routine.map(Routine).map(Target::Label),
        schema.map(Schema).map(Target::Label),
        sequence.map(Sequence).map(Target::Label),
        server.map(ForeignServer).map(Target::Label),
        statistics.map(ExtendedStatistics).map(Target::Label),
        subscription.map(Subscription).map(Target::Label),
        table.map(Table).map(Target::Label),
        tablespace.map(Tablespace).map(Target::Label),
        text_search.map(|text_search| match text_search {
            TextSearch::Configuration(name) => TextSearchConfiguration(name),
            TextSearch::Dictionary(name) => TextSearchDictionary(name),
            TextSearch::Parser(name) => TextSearchParser(name),
            TextSearch::Template(name) => TextSearchTemplate(name),
        }).map(Target::Label),
        type_name.map(Type).map(Target::Label),
        view.map(View).map(Target::Label),
    ).parse(ctx)
}

/// The `Option` result does not come from an absence of value.
/// It returns `None` when the token is the keyword `NULL`.
fn security_label(ctx: &mut ParserContext) -> scan::Result<Option<Box<str>>> {

    /*
          IS SCONST
        | IS NULL
    */

    let (_, label) = seq!(Is, string_or_null)
        .parse(ctx)?;

    Ok(label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::AggregateWithArgs,
        pg_ast::FunctionWithArgs,
        pg_ast::SignedNumber::IntegerConst,
        pg_ast::TypeName::Int4,
    };

    #[test_case(
        "SECURITY LABEL ON access method some_method IS 'foo'",
        SecurityLabelStmt::new(
            AccessMethod("some_method".into()),
            SecurityLabel::new(
                None,
                Some("foo".into())
            )
        )
    )]
    #[test_case(
        "SECURITY LABEL FOR 'some_label' ON access method some_method IS 'foo'",
        SecurityLabelStmt::new(
            AccessMethod("some_method".into()),
            SecurityLabel::new(
                Some("some_label".into()),
                Some("foo".into())
            )
        )
    )]
    fn test_security_label_stmt(source: &str, expected: SecurityLabelStmt) {
        test_parser!(source, security_label_stmt, expected)
    }

    #[test_case("for 'foo'", "foo".into())]
    fn test_provider(source: &str, expected: Str) {
        test_parser!(source, provider, expected);
    }

    #[test_case("access method some_method" => Ok(Target::Label(
        AccessMethod("some_method".into())
    )))]
    #[test_case("aggregate some_aggregate(*)" => Ok(Target::Label(
        Aggregate(AggregateWithArgs::new(
            vec!["some_aggregate".into()],
            vec![],
            vec![]
        ))
    )))]
    #[test_case("collation some_collation" => Ok(Target::Label(
        Collation(vec!["some_collation".into()])
    )))]
    #[test_case("column some_column" => Ok(Target::Label(
        Column(vec!["some_column".into()])
    )))]
    #[test_case("conversion some_conversion" => Ok(Target::Label(
        Conversion(vec!["some_conversion".into()])
    )))]
    #[test_case("database some_database" => Ok(
        Target::Database{ db_name: "some_database".into() }
    ))]
    #[test_case("domain int" => Ok(Target::Label(
        Domain(Int4.into())
    )))]
    #[test_case("event trigger some_trigger" => Ok(Target::Label(
        EventTrigger("some_trigger".into())
    )))]
    #[test_case("extension some_extension" => Ok(Target::Label(
        Extension("some_extension".into())
    )))]
    #[test_case("foreign data wrapper some_wrapper" => Ok(Target::Label(
        ForeignDataWrapper("some_wrapper".into())
    )))]
    #[test_case("foreign table some_table" => Ok(Target::Label(
        ForeignTable(vec!["some_table".into()])
    )))]
    #[test_case("function some_function" => Ok(Target::Label(
        Function(
            FunctionWithArgs::new(vec!["some_function".into()], None)
        )
    )))]
    #[test_case("index some_index" => Ok(Target::Label(
        Index(vec!["some_index".into()])
    )))]
    #[test_case("large object 123" => Ok(Target::Label(
        LargeObject(IntegerConst(123))
    )))]
    #[test_case("materialized view some_view" => Ok(Target::Label(
        MaterializedView(vec!["some_view".into()])
    )))]
    #[test_case("procedural language some_language" => Ok(Target::Label(
        Language("some_language".into())
    )))]
    #[test_case("language some_language" => Ok(Target::Label(
        Language("some_language".into())
    )))]
    #[test_case("procedure some_procedure" => Ok(Target::Label(
        Procedure(
            FunctionWithArgs::new(vec!["some_procedure".into()], None)
        )
    )))]
    #[test_case("property graph some_prop_graph" => Ok(Target::Label(
        PropertyGraph(vec!["some_prop_graph".into()])
    )))]
    #[test_case("publication some_publication" => Ok(Target::Label(
        Publication("some_publication".into())
    )))]
    #[test_case("role some_role" => Ok(Target::Label(
        Role("some_role".into())
    )))]
    #[test_case("routine some_routine" => Ok(Target::Label(
        Routine(
            FunctionWithArgs::new(vec!["some_routine".into()], None)
        )
    )))]
    #[test_case("schema some_schema" => Ok(Target::Label(
        Schema("some_schema".into())
    )))]
    #[test_case("sequence some_sequence" => Ok(Target::Label(
        Sequence(vec!["some_sequence".into()])
    )))]
    #[test_case("server some_server" => Ok(Target::Label(
        ForeignServer("some_server".into())
    )))]
    #[test_case("statistics some_statistics" => Ok(Target::Label(
        ExtendedStatistics(vec!["some_statistics".into()])
    )))]
    #[test_case("subscription some_subscription" => Ok(Target::Label(
        Subscription("some_subscription".into())
    )))]
    #[test_case("table some_table" => Ok(Target::Label(
        Table(vec!["some_table".into()])
    )))]
    #[test_case("tablespace some_tablespace" => Ok(Target::Label(
        Tablespace("some_tablespace".into())
    )))]
    #[test_case("text search configuration some_configuration" => Ok(Target::Label(
        TextSearchConfiguration(vec!["some_configuration".into()])
    )))]
    #[test_case("text search dictionary some_dictionary" => Ok(Target::Label(
        TextSearchDictionary(vec!["some_dictionary".into()])
    )))]
    #[test_case("text search parser some_parser" => Ok(Target::Label(
        TextSearchParser(vec!["some_parser".into()])
    )))]
    #[test_case("text search template some_template" => Ok(Target::Label(
        TextSearchTemplate(vec!["some_template".into()])
    )))]
    #[test_case("type int" => Ok(Target::Label(
        Type(Int4.into())
    )))]
    #[test_case("view some_view" => Ok(Target::Label(
        View(vec!["some_view".into()])
    )))]
    fn test_label_target(source: &str) -> scan::Result<Target> {
        test_parser!(source, label_target)
    }

    #[test_case("is 'abc'", Some("abc".into()))]
    #[test_case("is null", None)]
    fn test_comment_text(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, security_label, expected)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::non_reserved_word_or_sconst;
use crate::combinators::stmt::access_method;
use crate::combinators::stmt::aggregate;
use crate::combinators::stmt::collation;
use crate::combinators::stmt::column;
use crate::combinators::stmt::conversion;
use crate::combinators::stmt::database;
use crate::combinators::stmt::domain;
use crate::combinators::stmt::event_trigger;
use crate::combinators::stmt::extension;
use crate::combinators::stmt::foreign;
use crate::combinators::stmt::function;
use crate::combinators::stmt::index;
use crate::combinators::stmt::language;
use crate::combinators::stmt::large_object;
use crate::combinators::stmt::materialized_view;
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
use crate::combinators::stmt::type_name;
use crate::combinators::stmt::view;
use crate::combinators::stmt::Foreign;
use crate::combinators::stmt::TextSearch;
use crate::combinators::string_or_null;
use crate::seq;
use crate::ParserContext;
use pg_ast::DatabaseStmt;
use pg_ast::DatabaseStmtOption;
use pg_ast::RawStmt;
use pg_ast::SecurityLabel;
use pg_ast::SecurityLabelStmt;
use pg_ast::SecurityLabelTarget;
use pg_ast::SecurityLabelTarget::AccessMethod;
use pg_ast::SecurityLabelTarget::Aggregate;
use pg_ast::SecurityLabelTarget::Collation;
use pg_ast::SecurityLabelTarget::Column;
use pg_ast::SecurityLabelTarget::Conversion;
use pg_ast::SecurityLabelTarget::Domain;
use pg_ast::SecurityLabelTarget::EventTrigger;
use pg_ast::SecurityLabelTarget::ExtendedStatistics;
use pg_ast::SecurityLabelTarget::Extension;
use pg_ast::SecurityLabelTarget::ForeignDataWrapper;
use pg_ast::SecurityLabelTarget::ForeignServer;
use pg_ast::SecurityLabelTarget::ForeignTable;
use pg_ast::SecurityLabelTarget::Function;
use pg_ast::SecurityLabelTarget::Index;
use pg_ast::SecurityLabelTarget::Language;
use pg_ast::SecurityLabelTarget::LargeObject;
use pg_ast::SecurityLabelTarget::MaterializedView;
use pg_ast::SecurityLabelTarget::Procedure;
use pg_ast::SecurityLabelTarget::PropertyGraph;
use pg_ast::SecurityLabelTarget::Publication;
use pg_ast::SecurityLabelTarget::Role;
use pg_ast::SecurityLabelTarget::Routine;
use pg_ast::SecurityLabelTarget::Schema;
use pg_ast::SecurityLabelTarget::Sequence;
use pg_ast::SecurityLabelTarget::Subscription;
use pg_ast::SecurityLabelTarget::Table;
use pg_ast::SecurityLabelTarget::Tablespace;
use pg_ast::SecurityLabelTarget::TextSearchConfiguration;
use pg_ast::SecurityLabelTarget::TextSearchDictionary;
use pg_ast::SecurityLabelTarget::TextSearchParser;
use pg_ast::SecurityLabelTarget::TextSearchTemplate;
use pg_ast::SecurityLabelTarget::Type;
use pg_ast::SecurityLabelTarget::View;
use pg_basics::Str;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Is;
use pg_lexer::Keyword::Label;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::Security;
use pg_parser_core::scan;
