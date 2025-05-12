/// Alias: `SecLabelStmt`
pub(super) fn security_label_stmt() -> impl Combinator<Output = SecurityLabelStmt> {

    /*
        SECURITY LABEL opt_provider ON label_target IS security_label
    */

    sequence!(
        Security.and(Label).and_right(opt_provider()),
        On.and_right(label_target()),
        security_label()
    )
        .map(|(provider, target, label)| {
            SecurityLabelStmt::new(provider, target, label)
        })
}

fn opt_provider() -> impl Combinator<Output = Option<Str>> {

    /*
        ( FOR NonReservedWord_or_Sconst )?
    */

    For.and_right(non_reserved_word_or_sconst())
        .optional()
}

fn label_target() -> impl Combinator<Output = SecurityLabelTarget> {

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
      | opt_procedural LANGUAGE name
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

    match_first! {
        access_method().map(AccessMethod),
        aggregate().map(Aggregate),
        collation().map(Collation),
        column().map(Column),
        conversion().map(Conversion),
        database().map(Database),
        domain().map(Domain),
        event_trigger().map(EventTrigger),
        extension().map(Extension),
        foreign().map(|foreign| match foreign {
            Foreign::DataWrapper(name) => ForeignDataWrapper(name),
            Foreign::Table(name) => ForeignTable(name),
        }),
        function().map(Function),
        index().map(Index),
        large_object().map(LargeObject),
        materialized_view().map(MaterializedView),
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
        type_name().map(Type),
        view().map(View),
    }
}

fn security_label() -> impl Combinator<Output = Option<Box<str>>> {

    /*
          IS SCONST
        | IS NULL
    */

    Is.and_right(string_or_null())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::{
        AggregateWithArgs,
        FunctionWithArgs,
        SignedNumber::IntegerConst,
        TypeName::Int4,
    };
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_security_label_stmt() {
        test_parser!(
            source = "SECURITY LABEL ON access method some_method IS 'foo'",
            parser = security_label_stmt(),
            expected = SecurityLabelStmt::new(
                None,
                AccessMethod("some_method".into()),
                Some("foo".into())
            )
        )
    }

    #[test_case("", None)]
    #[test_case("for 'foo'", Some("foo".into()))]
    fn test_opt_provider(source: &str, expected: Option<Str>) {
        test_parser!(source, opt_provider(), expected);
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
        test_parser!(source, label_target(), expected)
    }

    #[test_case("is 'abc'", Some("abc".into()))]
    #[test_case("is null", None)]
    fn test_comment_text(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, security_label(), expected)
    }
}

use crate::parser::ast_node::SecurityLabelStmt;
use crate::parser::ast_node::SecurityLabelTarget;
use crate::parser::ast_node::SecurityLabelTarget::*;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::non_reserved_word_or_sconst;
use crate::parser::combinators::stmt::access_method;
use crate::parser::combinators::stmt::aggregate;
use crate::parser::combinators::stmt::collation;
use crate::parser::combinators::stmt::column;
use crate::parser::combinators::stmt::conversion;
use crate::parser::combinators::stmt::database;
use crate::parser::combinators::stmt::domain;
use crate::parser::combinators::stmt::event_trigger;
use crate::parser::combinators::stmt::extension;
use crate::parser::combinators::stmt::foreign;
use crate::parser::combinators::stmt::function;
use crate::parser::combinators::stmt::index;
use crate::parser::combinators::stmt::language;
use crate::parser::combinators::stmt::large_object;
use crate::parser::combinators::stmt::materialized_view;
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
use crate::parser::combinators::stmt::type_name;
use crate::parser::combinators::stmt::view;
use crate::parser::combinators::stmt::Foreign;
use crate::parser::combinators::stmt::TextSearch;
use crate::parser::combinators::string_or_null;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::For;
use postgres_parser_lexer::Keyword::Is;
use postgres_parser_lexer::Keyword::Label;
use postgres_parser_lexer::Keyword::On;
use postgres_parser_lexer::Keyword::Security;
