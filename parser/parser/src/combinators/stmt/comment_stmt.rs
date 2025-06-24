/// Alias: `CommentStmt`
pub(super) fn comment_stmt(stream: &mut TokenStream) -> Result<CommentStmt> {

    /*
          COMMENT ON comment_target IS comment_text
    */

    seq!(stream => Comment, On, comment_target, comment_text)
        .map(|(.., target, comment)|
            CommentStmt::new(target, comment)
        )
}

fn comment_target(stream: &mut TokenStream) -> Result<CommentTarget> {

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

    choice!(parsed stream =>
        access_method.map(AccessMethod),
        aggregate.map(Aggregate),
        typecast.map(Typecast),
        collation.map(Collation),
        column.map(Column),
        constraint,
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
        operator.map(|op| match op {
            Op::WithArgs(op) => Operator(op),
            Op::Class { name, index_method } => OperatorClass { name, index_method },
            Op::Family { name, index_method } => OperatorFamily { name, index_method },
        }),
        language.map(Language),
        policy,
        procedure.map(Procedure),
        publication.map(Publication),
        role.map(Role),
        routine.map(Routine),
        rule,
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
        trigger,
        type_name.map(Type),
        view.map(View),
    )
}

fn constraint(stream: &mut TokenStream) -> Result<CommentTarget> {

    enum Constraint {
        Domain(TypeName),
        Table(QualifiedName)
    }

    seq!(=>
        Kw::Constraint.parse(stream),
        col_id.parse(stream),
        On.parse(stream),
        choice!(stream =>
            // See https://github.com/postgres/postgres/blob/cdc168ad4b22ea4183f966688b245cabb5935d1f/src/backend/parser/gram.y#L7230-L7232
            seq!(stream => Kw::Domain, simple_typename())
                .map(|(_, domain)| Constraint::Domain(domain)),
            any_name.parse(stream)
                .map(Constraint::Table)
        )
    )
        .map(|(_, name, _, constraint)| match constraint {
            Constraint::Domain(domain) => DomainConstraint {
                constraint: name,
                domain,
            },
            Constraint::Table(table) => TableConstraint {
                constraint: name,
                table,
            },
        })
}

fn policy(stream: &mut TokenStream) -> Result<CommentTarget> {

    /*
        POLICY name ON any_name
    */

    seq!(stream => Kw::Policy, col_id, On, any_name)
        .map(|(_, name, _, table)| Policy { name, table })
}

fn rule(stream: &mut TokenStream) -> Result<CommentTarget> {

    /*
        RULE name ON any_name
    */

    seq!(stream => Kw::Rule, col_id, On, any_name)
        .map(|(_, name, _, table)| Rule { name, table })
}

fn trigger(stream: &mut TokenStream) -> Result<CommentTarget> {

    /*
        TRIGGER name ON any_name
    */

    seq!(stream => Kw::Trigger, col_id, On, any_name)
        .map(|(_, name, _, table)| Trigger { name, table })
}

fn comment_text(stream: &mut TokenStream) -> Result<Option<Box<str>>> {

    /*
          IS SCONST
        | IS NULL
    */

    seq!(stream => Is, string_or_null)
        .map(|(_, text)| text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
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
    use test_case::test_case;

    #[test]
    fn test_comment_stmt() {
        test_parser!(
            source = "comment on schema foo is 'bar'",
            parser = comment_stmt,
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
        test_parser!(source, comment_target, expected)
    }

    #[test_case("is 'abc'", Some("abc".into()))]
    #[test_case("is null", None)]
    fn test_comment_text(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, comment_text, expected)
    }
}

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::simple_typename;
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
use crate::combinators::stmt::operator;
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
use crate::combinators::stmt::transform;
use crate::combinators::stmt::type_name;
use crate::combinators::stmt::typecast;
use crate::combinators::stmt::view;
use crate::combinators::stmt::Foreign;
use crate::combinators::stmt::Operator as Op;
use crate::combinators::stmt::TextSearch;
use crate::combinators::string_or_null;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::CommentStmt;
use pg_ast::CommentTarget;
use pg_ast::CommentTarget::AccessMethod;
use pg_ast::CommentTarget::Aggregate;
use pg_ast::CommentTarget::Collation;
use pg_ast::CommentTarget::Column;
use pg_ast::CommentTarget::Conversion;
use pg_ast::CommentTarget::Database;
use pg_ast::CommentTarget::Domain;
use pg_ast::CommentTarget::DomainConstraint;
use pg_ast::CommentTarget::EventTrigger;
use pg_ast::CommentTarget::ExtendedStatistics;
use pg_ast::CommentTarget::Extension;
use pg_ast::CommentTarget::ForeignDataWrapper;
use pg_ast::CommentTarget::ForeignServer;
use pg_ast::CommentTarget::ForeignTable;
use pg_ast::CommentTarget::Function;
use pg_ast::CommentTarget::Index;
use pg_ast::CommentTarget::Language;
use pg_ast::CommentTarget::LargeObject;
use pg_ast::CommentTarget::MaterializedView;
use pg_ast::CommentTarget::Operator;
use pg_ast::CommentTarget::OperatorClass;
use pg_ast::CommentTarget::OperatorFamily;
use pg_ast::CommentTarget::Policy;
use pg_ast::CommentTarget::Procedure;
use pg_ast::CommentTarget::Publication;
use pg_ast::CommentTarget::Role;
use pg_ast::CommentTarget::Routine;
use pg_ast::CommentTarget::Rule;
use pg_ast::CommentTarget::Schema;
use pg_ast::CommentTarget::Sequence;
use pg_ast::CommentTarget::Subscription;
use pg_ast::CommentTarget::Table;
use pg_ast::CommentTarget::TableConstraint;
use pg_ast::CommentTarget::Tablespace;
use pg_ast::CommentTarget::TextSearchConfiguration;
use pg_ast::CommentTarget::TextSearchDictionary;
use pg_ast::CommentTarget::TextSearchParser;
use pg_ast::CommentTarget::TextSearchTemplate;
use pg_ast::CommentTarget::Transform;
use pg_ast::CommentTarget::Trigger;
use pg_ast::CommentTarget::Type;
use pg_ast::CommentTarget::Typecast;
use pg_ast::CommentTarget::View;
use pg_ast::TypeName;
use pg_basics::QualifiedName;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Comment;
use pg_lexer::Keyword::Is;
use pg_lexer::Keyword::On;
