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
        access_method(),
        aggregate(),
        collation(),
        column(),
        conversion(),
        database(),
        domain(),
        event_trigger(),
        extension(),
        foreign(),
        function(),
        index(),
        large_object(),
        materialized_view(),
        language(),
        procedure(),
        publication(),
        role(),
        routine(),
        schema(),
        sequence(),
        server(),
        statistics(),
        subscription(),
        table(),
        tablespace(),
        text_search(),
        type_(),
        view(),
    }
}

fn access_method() -> impl Combinator<Output = SecurityLabelTarget> {
    and(Access, Method)
        .and_right(col_id())
        .map(AccessMethod)
}

fn aggregate() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Aggregate
        .and_right(aggregate_with_argtypes())
        .map(Aggregate)
}

fn collation() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Collation
        .and_right(any_name())
        .map(Collation)
}

fn column() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Column
        .and_right(any_name())
        .map(Column)
}

fn conversion() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Conversion
        .and_right(any_name())
        .map(Conversion)
}

fn database() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Database
        .and_right(col_id())
        .map(Database)
}

fn domain() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Domain
        .and_right(typename())
        .map(Domain)
}

fn event_trigger() -> impl Combinator<Output = SecurityLabelTarget> {
    and(Event, Kw::Trigger)
        .and_right(col_id())
        .map(EventTrigger)
}

fn extension() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Extension
        .and_right(col_id())
        .map(Extension)
}

fn foreign() -> impl Combinator<Output = SecurityLabelTarget> {
    Foreign.and_right(or(
        and(Data, Wrapper)
            .and_right(col_id())
            .map(ForeignDataWrapper),
        Kw::Table
            .and_right(any_name())
            .map(ForeignTable)
    ))
}

fn function() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Function
        .and_right(function_with_argtypes())
        .map(Function)
}

fn index() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Index
        .and_right(any_name())
        .map(Index)
}

fn large_object() -> impl Combinator<Output = SecurityLabelTarget> {
    and(Large, Object)
        .and_right(signed_number())
        .map(LargeObject)
}

fn materialized_view() -> impl Combinator<Output = SecurityLabelTarget> {
    and(Kw::Materialized, Kw::View)
        .and_right(any_name())
        .map(MaterializedView)
}

fn language() -> impl Combinator<Output = SecurityLabelTarget> {
    or(
        Kw::Language.skip(),
        and(Procedural, Kw::Language).skip()
    )
        .and_right(col_id())
        .map(Language)
}

fn procedure() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Procedure
        .and_right(function_with_argtypes())
        .map(Procedure)
}

fn publication() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Publication
        .and_right(col_id())
        .map(Publication)
}

fn role() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Role
        .and_right(col_id())
        .map(Role)
}

fn routine() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Routine
        .and_right(function_with_argtypes())
        .map(Routine)
}

fn schema() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Schema
        .and_right(col_id())
        .map(Schema)
}

fn sequence() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Sequence
        .and_right(any_name())
        .map(Sequence)
}

fn server() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Server
        .and_right(col_id())
        .map(ForeignServer)
}

fn statistics() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Statistics
        .and_right(any_name())
        .map(ExtendedStatistics)
}

fn subscription() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Subscription
        .and_right(col_id())
        .map(Subscription)
}

fn table() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Table
        .and_right(any_name())
        .map(Table)
}

fn tablespace() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Tablespace
        .and_right(col_id())
        .map(Tablespace)
}

fn text_search() -> impl Combinator<Output = SecurityLabelTarget> {
    and(Text, Search)
        .and_right(match_first! {
            Configuration
                .and_right(any_name())
                .map(TextSearchConfiguration),
            Dictionary
                .and_right(any_name())
                .map(TextSearchDictionary),
            ParserKw
                .and_right(any_name())
                .map(TextSearchParser),
            Template
                .and_right(any_name())
                .map(TextSearchTemplate)
        })
}

fn type_() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::Type
        .and_right(typename())
        .map(Type)
}

fn view() -> impl Combinator<Output = SecurityLabelTarget> {
    Kw::View
        .and_right(any_name())
        .map(View)
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

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::Access;
use crate::lexer::Keyword::Configuration;
use crate::lexer::Keyword::Data;
use crate::lexer::Keyword::Dictionary;
use crate::lexer::Keyword::Event;
use crate::lexer::Keyword::For;
use crate::lexer::Keyword::Foreign;
use crate::lexer::Keyword::Is;
use crate::lexer::Keyword::Label;
use crate::lexer::Keyword::Large;
use crate::lexer::Keyword::Method;
use crate::lexer::Keyword::Object;
use crate::lexer::Keyword::On;
use crate::lexer::Keyword::ParserKw;
use crate::lexer::Keyword::Procedural;
use crate::lexer::Keyword::Search;
use crate::lexer::Keyword::Security;
use crate::lexer::Keyword::Template;
use crate::lexer::Keyword::Text;
use crate::lexer::Keyword::Wrapper;
use crate::parser::ast_node::SecurityLabelStmt;
use crate::parser::ast_node::SecurityLabelTarget;
use crate::parser::ast_node::SecurityLabelTarget::*;
use crate::parser::combinators::any_name;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes;
use crate::parser::combinators::non_reserved_word_or_sconst;
use crate::parser::combinators::signed_number;
use crate::parser::combinators::stmt::aggregate_with_argtypes;
use crate::parser::combinators::string_or_null;
use crate::parser::combinators::typename;
use postgres_basics::Str;
