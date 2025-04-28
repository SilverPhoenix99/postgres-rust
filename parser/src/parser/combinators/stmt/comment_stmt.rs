/// Alias: `CommentStmt`
pub(super) fn comment_stmt() -> impl Combinator<Output = CommentStmt> {

    /*
          COMMENT ON comment_target IS comment_text
    */

    Comment.and(On)
        .and_right(comment_target())
        .and_then(comment_text(), CommentStmt::new)
}

fn comment_target() -> impl Combinator<Output = CommentTarget> {

    /*
          ACCESS METHOD name
        | AGGREGATE aggregate_with_argtypes
        | CAST '(' Typename AS Typename ')'
        | COLLATION any_name
        | COLUMN any_name
        | CONSTRAINT name ON any_name
        | CONSTRAINT name ON DOMAIN_P any_name
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
        | OPERATOR CLASS any_name USING name
        | OPERATOR FAMILY any_name USING name
        | OPERATOR operator_with_argtypes
        | opt_procedural LANGUAGE name
        | POLICY name ON any_name
        | PROCEDURE function_with_argtypes
        | PUBLICATION name
        | ROLE name
        | ROUTINE function_with_argtypes
        | RULE name ON any_name
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
        | TRANSFORM FOR Typename LANGUAGE name
        | TRIGGER name ON any_name
        | TYPE_P Typename
        | VIEW any_name
    */

    match_first! {
        access_method().map(AccessMethod),
        aggregate().map(Aggregate),
        typecast().map(Typecast),
        collation().map(Collation),
        column().map(Column),
        constraint(),
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
        operator().map(|op| match op {
            Op::WithArgs(op) => Operator(op),
            Op::Class { name, index_method } => OperatorClass { name, index_method },
            Op::Family { name, index_method } => OperatorFamily { name, index_method },
        }),
        language().map(Language),
        policy(),
        procedure().map(Procedure),
        publication().map(Publication),
        role().map(Role),
        routine().map(Routine),
        rule(),
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
        trigger(),
        type_name().map(Type),
        view().map(View),
    }
}

fn constraint() -> impl Combinator<Output = CommentTarget> {
    Kw::Constraint
        .and_right(col_id())
        .and_left(On)
        .chain(match_first_with_state!(|constraint, stream| {
            // See https://github.com/postgres/postgres/blob/cdc168ad4b22ea4183f966688b245cabb5935d1f/src/backend/parser/gram.y#L7230-L7232
            Kw::Domain.and_right(simple_typename()) => (domain) DomainConstraint { constraint, domain },
            any_name() => (table) TableConstraint { constraint, table },
        }))
}

fn policy() -> impl Combinator<Output = CommentTarget> {
    Kw::Policy
        .and_right(col_id())
        .and_then(
            On.and_right(any_name()),
            |name, table| Policy { name, table }
        )
}

fn rule() -> impl Combinator<Output = CommentTarget> {
    Kw::Rule
        .and_right(col_id())
        .and_then(
            On.and_right(any_name()),
            |name, table| Rule { name, table }
        )
}

fn trigger() -> impl Combinator<Output = CommentTarget> {
    Kw::Trigger
        .and_right(col_id())
        .and_then(
            On.and_right(any_name()),
            |name, table| Trigger { name, table }
        )
}

fn comment_text() -> impl Combinator<Output = Option<Box<str>>> {

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
        OneOrBoth,
        Operator::Addition,
        OperatorWithArgs,
        QualifiedOperator,
        SignedNumber::IntegerConst,
        Transform as TransformAst,
        TypeName::{Int4, Varchar},
        Typecast as Cast,
    };
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test]
    fn test_comment_stmt() {
        test_parser!(
            source = "comment on schema foo is 'bar'",
            parser = comment_stmt(),
            expected = CommentStmt::new(
                Schema("foo".into()),
                Some("bar".into())
            )
        )
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
    #[test_case("column some_column", Column(vec!["some_column".into()]))]
    #[test_case("constraint some_constraint on domain int",
        DomainConstraint {
            constraint: "some_constraint".into(),
            domain: Int4
        }
    )]
    #[test_case("constraint some_constraint on some_table",
        TableConstraint {
            constraint: "some_constraint".into(),
            table: vec!["some_table".into()]
        }
    )]
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
    #[test_case("operator class some_class using some_method",
        OperatorClass {
            name: vec!["some_class".into()],
            index_method: "some_method".into()
        }
    )]
    #[test_case("operator family some_family using some_method",
        OperatorFamily {
            name: vec!["some_family".into()],
            index_method: "some_method".into()
        }
    )]
    #[test_case("operator +(int, int)", Operator(
        OperatorWithArgs::new(
            QualifiedOperator(vec![], Addition),
            OneOrBoth::Both(Int4.into(), Int4.into())
        )
    ))]
    #[test_case("procedural language some_language", Language("some_language".into()))]
    #[test_case("language some_language", Language("some_language".into()))]
    #[test_case("policy some_policy on some_table",
        Policy {
            name: "some_policy".into(),
            table: vec!["some_table".into()]
        }
    )]
    #[test_case("procedure some_procedure", Procedure(
        FunctionWithArgs::new(vec!["some_procedure".into()], None)
    ))]
    #[test_case("publication some_publication", Publication("some_publication".into()))]
    #[test_case("role some_role", Role("some_role".into()))]
    #[test_case("routine some_routine", Routine(
        FunctionWithArgs::new(vec!["some_routine".into()], None)
    ))]
    #[test_case("rule some_rule on some_table",
        Rule {
            name: "some_rule".into(),
            table: vec!["some_table".into()]
        }
    )]
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
    #[test_case("trigger some_trigger on some_table",
        Trigger {
            name: "some_trigger".into(),
            table: vec!["some_table".into()]
        }
    )]
    #[test_case("type int", Type(Int4.into()))]
    #[test_case("view some_view", View(vec!["some_view".into()]))]
    fn test_comment_target(source: &str, expected: CommentTarget) {
        test_parser!(source, comment_target(), expected)
    }

    #[test_case("is 'abc'", Some("abc".into()))]
    #[test_case("is null", None)]
    fn test_comment_text(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, comment_text(), expected)
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::Comment;
use crate::lexer::Keyword::Is;
use crate::lexer::Keyword::On;
use crate::parser::ast_node::CommentStmt;
use crate::parser::ast_node::CommentTarget;
use crate::parser::ast_node::CommentTarget::*;
use crate::parser::combinators::any_name;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::simple_typename;
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
use crate::parser::combinators::string_or_null;
