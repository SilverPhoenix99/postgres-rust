/// Alias: `CommentStmt`
pub(super) fn comment_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
          COMMENT ON comment_target IS comment_text
    */

    let (.., target, comment) = seq!(Comment, On, comment_target, comment_text)
        .parse(ctx)?;

    let stmt = match target {
        Target::Database { db_name } => {
            let option = DatabaseStmtOption::Comment(comment);
            DatabaseStmt::new(db_name, option).into()
        }
        Target::Comment(target) => {
            CommentStmt::new(target, comment).into()
        }
    };

    Ok(stmt)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Target {
    Database { db_name: Str },
    Comment(CommentTarget),
}

fn comment_target(ctx: &mut ParserContext) -> scan::Result<Target> {

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
        | ( PROCEDURAL )? LANGUAGE name
        | POLICY name ON any_name
        | PROCEDURE function_with_argtypes
        | PROPERTY GRAPH any_name
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

    alt!(
        access_method.map(AccessMethod).map(Target::Comment),
        aggregate.map(Aggregate).map(Target::Comment),
        typecast.map(Typecast).map(Target::Comment),
        collation.map(Collation).map(Target::Comment),
        column.map(Column).map(Target::Comment),
        constraint,
        conversion.map(Conversion).map(Target::Comment),
        database.map(|db_name| {
            Target::Database { db_name }
        }),
        domain.map(Domain).map(Target::Comment),
        event_trigger.map(EventTrigger).map(Target::Comment),
        extension.map(Extension).map(Target::Comment),
        foreign.map(|foreign| match foreign {
            Foreign::DataWrapper(name) => ForeignDataWrapper(name),
            Foreign::Table(name) => ForeignTable(name),
        }).map(Target::Comment),
        function.map(Function).map(Target::Comment),
        index.map(Index).map(Target::Comment),
        large_object.map(LargeObject).map(Target::Comment),
        materialized_view.map(MaterializedView).map(Target::Comment),
        operator.map(|op| match op {
            Op::WithArgs(op) => Operator(op),
            Op::Class { name, index_method } => OperatorClass { name, index_method },
            Op::Family { name, index_method } => OperatorFamily { name, index_method },
        }).map(Target::Comment),
        language.map(Language).map(Target::Comment),
        policy,
        procedure.map(Procedure).map(Target::Comment),
        property_graph.map(PropertyGraph).map(Target::Comment),
        publication.map(Publication).map(Target::Comment),
        role.map(Role).map(Target::Comment),
        routine.map(Routine).map(Target::Comment),
        rule,
        schema.map(Schema).map(Target::Comment),
        sequence.map(Sequence).map(Target::Comment),
        server.map(ForeignServer).map(Target::Comment),
        statistics.map(ExtendedStatistics).map(Target::Comment),
        subscription.map(Subscription).map(Target::Comment),
        table.map(Table).map(Target::Comment),
        tablespace.map(Tablespace).map(Target::Comment),
        text_search.map(|text_search| match text_search {
            TextSearch::Configuration(name) => TextSearchConfiguration(name),
            TextSearch::Dictionary(name) => TextSearchDictionary(name),
            TextSearch::Parser(name) => TextSearchParser(name),
            TextSearch::Template(name) => TextSearchTemplate(name),
        }).map(Target::Comment),
        transform.map(Transform).map(Target::Comment),
        trigger,
        type_name.map(Type).map(Target::Comment),
        view.map(View).map(Target::Comment),
    ).parse(ctx)
}

fn constraint(ctx: &mut ParserContext) -> scan::Result<Target> {

    enum Constraint {
        Domain(TypeName),
        Table(QualifiedName)
    }

    let (_, name, _, constraint) = seq!(
        Kw::Constraint,
        col_id,
        On,
        alt!(
            // See https://github.com/postgres/postgres/blob/cdc168ad4b22ea4183f966688b245cabb5935d1f/src/backend/parser/gram.y#L7230-L7232
            seq!(Kw::Domain, simple_typename)
                .map(|(_, domain)| Constraint::Domain(domain)),
            any_name
                .map(Constraint::Table)
        )
    ).parse(ctx)?;

    let target = match constraint {
        Constraint::Domain(domain) => DomainConstraint {
            constraint: name,
            domain,
        },
        Constraint::Table(table) => TableConstraint {
            constraint: name,
            table,
        },
    };

    Ok(Target::Comment(target))
}

fn policy(ctx: &mut ParserContext) -> scan::Result<Target> {

    /*
        POLICY name ON any_name
    */

    let (_, name, _, table) = seq!(Kw::Policy, col_id, On, any_name)
        .parse(ctx)?;

    let target = Policy { name, table };
    Ok(Target::Comment(target))
}

fn rule(ctx: &mut ParserContext) -> scan::Result<Target> {

    /*
        RULE name ON any_name
    */

    let (_, name, _, table) = seq!(Kw::Rule, col_id, On, any_name)
        .parse(ctx)?;

    let target = Rule { name, table };
    Ok(Target::Comment(target))
}

fn trigger(ctx: &mut ParserContext) -> scan::Result<Target> {

    /*
        TRIGGER name ON any_name
    */

    let (_, name, _, table) = seq!(Kw::Trigger, col_id, On, any_name)
        .parse(ctx)?;

    let target = Trigger { name, table };
    Ok(Target::Comment(target))
}

/// The `Option` result does not come from an absence of value.
/// It returns `None` when the token is the keyword `NULL`.
fn comment_text(ctx: &mut ParserContext) -> scan::Result<Option<Box<str>>> {

    /*
          IS SCONST
        | IS NULL
    */

    let (_, text) = seq!(Is, string_or_null)
        .parse(ctx)?;

    Ok(text)
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
        pg_ast::OneOrBoth,
        pg_ast::Operator::Addition,
        pg_ast::OperatorWithArgs,
        pg_ast::QualifiedOperator,
        pg_ast::SignedNumber::IntegerConst,
        pg_ast::Transform as TransformAst,
        pg_ast::TypeName::Int4,
        pg_ast::TypeName::Varchar,
        pg_ast::Typecast as Cast,
    };

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

    #[test_case("access method some_method" => Ok(Target::Comment(
        AccessMethod("some_method".into())
    )))]
    #[test_case("aggregate some_aggregate(*)" => Ok(Target::Comment(
        Aggregate(AggregateWithArgs::new(
            vec!["some_aggregate".into()],
            vec![],
            vec![]
        ))
    )))]
    #[test_case("cast (int as varchar)" => Ok(Target::Comment(
        Typecast(Cast::new(
            Int4,
            Varchar { max_length: None }
        ))
    )))]
    #[test_case("collation some_collation" => Ok(Target::Comment(
        Collation(vec!["some_collation".into()])
    )))]
    #[test_case("column some_column" => Ok(Target::Comment(
        Column(vec!["some_column".into()])
    )))]
    #[test_case("constraint some_constraint on domain int" => Ok(Target::Comment(
        DomainConstraint {
            constraint: "some_constraint".into(),
            domain: Int4
        }
    )))]
    #[test_case("constraint some_constraint on some_table" => Ok(Target::Comment(
        TableConstraint {
            constraint: "some_constraint".into(),
            table: vec!["some_table".into()]
        }
    )))]
    #[test_case("conversion some_conversion" => Ok(Target::Comment(
        Conversion(vec!["some_conversion".into()])
    )))]
    #[test_case("database some_database" => Ok(
        Target::Database { db_name: "some_database".into() }
    ))]
    #[test_case("domain int" => Ok(Target::Comment(
        Domain(Int4.into())
    )))]
    #[test_case("event trigger some_trigger" => Ok(Target::Comment(
        EventTrigger("some_trigger".into())
    )))]
    #[test_case("extension some_extension" => Ok(Target::Comment(
        Extension("some_extension".into())
    )))]
    #[test_case("foreign data wrapper some_wrapper" => Ok(Target::Comment(
        ForeignDataWrapper("some_wrapper".into())
    )))]
    #[test_case("foreign table some_table" => Ok(Target::Comment(
        ForeignTable(vec!["some_table".into()])
    )))]
    #[test_case("function some_function" => Ok(Target::Comment(
        Function(
            FunctionWithArgs::new(vec!["some_function".into()], None)
        )
    )))]
    #[test_case("index some_index" => Ok(Target::Comment(
        Index(vec!["some_index".into()])
    )))]
    #[test_case("large object 123" => Ok(Target::Comment(
        LargeObject(IntegerConst(123))
    )))]
    #[test_case("materialized view some_view" => Ok(Target::Comment(
        MaterializedView(vec!["some_view".into()])
    )))]
    #[test_case("operator class some_class using some_method" => Ok(Target::Comment(
        OperatorClass {
            name: vec!["some_class".into()],
            index_method: "some_method".into()
        }
    )))]
    #[test_case("operator family some_family using some_method" => Ok(Target::Comment(
        OperatorFamily {
            name: vec!["some_family".into()],
            index_method: "some_method".into()
        }
    )))]
    #[test_case("operator +(int, int)" => Ok(Target::Comment(Operator(
        OperatorWithArgs::new(
            QualifiedOperator(vec![], Addition),
            OneOrBoth::Both(Int4.into(), Int4.into())
        )
    ))))]
    #[test_case("procedural language some_language" => Ok(Target::Comment(
        Language("some_language".into())
    )))]
    #[test_case("language some_language" => Ok(Target::Comment(
        Language("some_language".into())
    )))]
    #[test_case("policy some_policy on some_table" => Ok(Target::Comment(
        Policy {
            name: "some_policy".into(),
            table: vec!["some_table".into()]
        }
    )))]
    #[test_case("procedure some_procedure" => Ok(Target::Comment(
        Procedure(
            FunctionWithArgs::new(vec!["some_procedure".into()], None)
        )
    )))]
    #[test_case("property graph some_prop_graph" => Ok(Target::Comment(
        PropertyGraph(vec!["some_prop_graph".into()])
    )))]
    #[test_case("publication some_publication" => Ok(Target::Comment(
        Publication("some_publication".into())
    )))]
    #[test_case("role some_role" => Ok(Target::Comment(
        Role("some_role".into())
    )))]
    #[test_case("routine some_routine" => Ok(Target::Comment(
        Routine(
            FunctionWithArgs::new(vec!["some_routine".into()], None)
        )
    )))]
    #[test_case("rule some_rule on some_table" => Ok(Target::Comment(
        Rule {
            name: "some_rule".into(),
            table: vec!["some_table".into()]
        }
    )))]
    #[test_case("schema some_schema" => Ok(Target::Comment(
        Schema("some_schema".into())
    )))]
    #[test_case("sequence some_sequence" => Ok(Target::Comment(
        Sequence(vec!["some_sequence".into()])
    )))]
    #[test_case("server some_server" => Ok(Target::Comment(
        ForeignServer("some_server".into())
    )))]
    #[test_case("statistics some_statistics" => Ok(Target::Comment(
        ExtendedStatistics(vec!["some_statistics".into()])
    )))]
    #[test_case("subscription some_subscription" => Ok(Target::Comment(
        Subscription("some_subscription".into())
    )))]
    #[test_case("table some_table" => Ok(Target::Comment(
        Table(vec!["some_table".into()])
    )))]
    #[test_case("tablespace some_tablespace" => Ok(Target::Comment(
        Tablespace("some_tablespace".into())
    )))]
    #[test_case("text search configuration some_configuration" => Ok(Target::Comment(
        TextSearchConfiguration(vec!["some_configuration".into()])
    )))]
    #[test_case("text search dictionary some_dictionary" => Ok(Target::Comment(
        TextSearchDictionary(vec!["some_dictionary".into()])
    )))]
    #[test_case("text search parser some_parser" => Ok(Target::Comment(
        TextSearchParser(vec!["some_parser".into()])
    )))]
    #[test_case("text search template some_template" => Ok(Target::Comment(
        TextSearchTemplate(vec!["some_template".into()])
    )))]
    #[test_case("transform for int language some_language" => Ok(Target::Comment(
        Transform(TransformAst::new(Int4, "some_language"))
    )))]
    #[test_case("trigger some_trigger on some_table" => Ok(Target::Comment(
        Trigger {
            name: "some_trigger".into(),
            table: vec!["some_table".into()]
        }
    )))]
    #[test_case("type int" => Ok(Target::Comment(
        Type(Int4.into())
    )))]
    #[test_case("view some_view" => Ok(Target::Comment(
        View(vec!["some_view".into()])
    )))]
    fn test_comment_target(source: &str) -> scan::Result<Target> {
        test_parser!(source, comment_target)
    }

    #[test_case("is 'abc'", Some("abc".into()))]
    #[test_case("is null", None)]
    fn test_comment_text(source: &str, expected: Option<Box<str>>) {
        test_parser!(source, comment_text, expected)
    }
}

use crate::alt;
use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::core::Combinator;
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
use crate::combinators::string_or_null;
use crate::seq;
use crate::ParserContext;
use pg_ast::CommentStmt;
use pg_ast::CommentTarget;
use pg_ast::CommentTarget::AccessMethod;
use pg_ast::CommentTarget::Aggregate;
use pg_ast::CommentTarget::Collation;
use pg_ast::CommentTarget::Column;
use pg_ast::CommentTarget::Conversion;
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
use pg_ast::CommentTarget::PropertyGraph;
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
use pg_ast::DatabaseStmt;
use pg_ast::DatabaseStmtOption;
use pg_ast::RawStmt;
use pg_ast::TypeName;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Comment;
use pg_lexer::Keyword::Is;
use pg_lexer::Keyword::On;
use pg_parser_core::scan;
