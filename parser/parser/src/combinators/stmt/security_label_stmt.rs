/// Alias: `SecLabelStmt`
pub(super) fn security_label_stmt(stream: &mut TokenStream) -> scan::Result<SecurityLabelStmt> {

    /*
        SECURITY LABEL ( provider )? ON label_target IS security_label
    */

    let (_, _, provider, _, target, label) = (
        Security,
        Label,
        provider.optional(),
        On,
        label_target,
        security_label
    ).parse(stream)?;

    let stmt = SecurityLabelStmt::new(provider, target, label);

    Ok(stmt)
}

/// Alias: `opt_provider`
fn provider(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        FOR NonReservedWord_or_Sconst
    */

    let (_, provider) = (For, non_reserved_word_or_sconst).parse(stream)?;

    Ok(provider)
}

fn label_target(stream: &mut TokenStream) -> scan::Result<SecurityLabelTarget> {

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

    // Broken down into smaller combinators, due to large Rust type names.
    alt!(
        access_method.map(AccessMethod),
        aggregate.map(Aggregate),
        collation.map(Collation),
        column.map(Column),
        conversion.map(Conversion),
        database.map(Database),
        domain.map(Domain),
        event_trigger.map(EventTrigger),
        extension.map(Extension),
        foreign.map(|foreign| match foreign {
            Foreign::DataWrapper(name) => ForeignDataWrapper(name),
            Foreign::Table(name) => ForeignTable(name),
        }),
        function.map(Function),
        index.map(Index),
        large_object.map(LargeObject),
        materialized_view.map(MaterializedView),
        language.map(Language),
        procedure.map(Procedure),
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
        type_name.map(Type),
        view.map(View),
    ).parse(stream)
}

/// The `Option` result does not come from an absence of value.
/// It returns `None` when the token is the keyword `NULL`.
fn security_label(stream: &mut TokenStream) -> scan::Result<Option<Box<str>>> {

    /*
          IS SCONST
        | IS NULL
    */

    let (_, label) = (Is, string_or_null)
        .parse(stream)?;

    Ok(label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        AggregateWithArgs,
        FunctionWithArgs,
        SignedNumber::IntegerConst,
        TypeName::Int4,
    };
    use test_case::test_case;

    #[test_case(
        "SECURITY LABEL ON access method some_method IS 'foo'",
        SecurityLabelStmt::new(
            None,
            AccessMethod("some_method".into()),
            Some("foo".into())
        )
    )]
    #[test_case(
        "SECURITY LABEL FOR 'some_label' ON access method some_method IS 'foo'",
        SecurityLabelStmt::new(
            Some("some_label".into()),
            AccessMethod("some_method".into()),
            Some("foo".into())
        )
    )]
    fn test_security_label_stmt(source: &str, expected: SecurityLabelStmt) {
        test_parser!(source, security_label_stmt, expected)
    }

    #[test_case("for 'foo'", "foo".into())]
    fn test_provider(source: &str, expected: Str) {
        test_parser!(source, provider, expected);
    }

    #[test_case("access method some_method", AccessMethod("some_method".into()))]
    #[test_case("aggregate some_aggregate(*)",
        Aggregate(AggregateWithArgs::new(
            vec!["some_aggregate".into()],
            vec![],
            vec![]
        ))
    )]
    #[test_case("collation some_collation", Collation(vec!["some_collation".into()]))]
    #[test_case("column some_column", Column(vec!["some_column".into()]))]
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
    #[test_case("large object 123", LargeObject(IntegerConst(123)))]
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
    #[test_case("type int", Type(Int4.into()))]
    #[test_case("view some_view", View(vec!["some_view".into()]))]
    fn test_label_target(source: &str, expected: SecurityLabelTarget) {
        test_parser!(source, label_target, expected)
    }

    #[test_case("is 'abc'", Some("abc".into()))]
    #[test_case("is null", None)]
    fn test_comment_text(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, security_label, expected)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::Combinator;
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
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::SecurityLabelStmt;
use pg_ast::SecurityLabelTarget;
use pg_ast::SecurityLabelTarget::AccessMethod;
use pg_ast::SecurityLabelTarget::Aggregate;
use pg_ast::SecurityLabelTarget::Collation;
use pg_ast::SecurityLabelTarget::Column;
use pg_ast::SecurityLabelTarget::Conversion;
use pg_ast::SecurityLabelTarget::Database;
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
