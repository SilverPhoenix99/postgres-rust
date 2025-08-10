/// Alias: `CommentStmt`
pub(super) fn comment_stmt(ctx: &mut ParserContext) -> scan::Result<CommentStmt> {

    /*
          COMMENT ON comment_target IS comment_text
    */

    let (.., target, comment) = seq!(Comment, On, comment_target, comment_text)
        .parse(ctx)?;

    Ok(CommentStmt::new(target, comment))
}

fn comment_target(ctx: &mut ParserContext) -> scan::Result<CommentTarget> {

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
        alt!(
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
        ),
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
    ).parse(ctx)
}

fn constraint(ctx: &mut ParserContext) -> scan::Result<CommentTarget> {

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

    Ok(target)
}

fn policy(ctx: &mut ParserContext) -> scan::Result<CommentTarget> {

    /*
        POLICY name ON any_name
    */

    let (_, name, _, table) = seq!(Kw::Policy, col_id, On, any_name)
        .parse(ctx)?;

    Ok(Policy { name, table })
}

fn rule(ctx: &mut ParserContext) -> scan::Result<CommentTarget> {

    /*
        RULE name ON any_name
    */

    let (_, name, _, table) = seq!(Kw::Rule, col_id, On, any_name)
        .parse(ctx)?;

    Ok(Rule { name, table })
}

fn trigger(ctx: &mut ParserContext) -> scan::Result<CommentTarget> {

    /*
        TRIGGER name ON any_name
    */

    let (_, name, _, table) = seq!(Kw::Trigger, col_id, On, any_name)
        .parse(ctx)?;

    Ok(Trigger { name, table })
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
    use pg_combinators::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::AggregateWithArgs,
        pg_ast::FunctionWithArgs,
        pg_ast::OneOrBoth,
        pg_ast::OperatorWithArgs,
        pg_ast::Transform as TransformAst,
        pg_ast::TypeName::Int4,
        pg_ast::TypeName::Varchar,
        pg_ast::Typecast as Cast,
        pg_sink_ast::Operator::Addition,
        pg_sink_ast::QualifiedOperator,
        pg_sink_ast::SignedNumber::IntegerConst,
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

use crate::combinators::stmt::access_method;
use crate::combinators::stmt::aggregate;
use crate::combinators::stmt::domain;
use crate::combinators::stmt::function;
use crate::combinators::stmt::operator;
use crate::combinators::stmt::procedure;
use crate::combinators::stmt::routine;
use crate::combinators::stmt::transform;
use crate::combinators::stmt::type_name;
use crate::combinators::stmt::typecast;
use crate::combinators::stmt::Operator as Op;
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
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Comment;
use pg_lexer::Keyword::Is;
use pg_lexer::Keyword::On;
use pg_parser_core::scan;
use pg_sink_combinators::any_name;
use pg_sink_combinators::col_id;
use pg_sink_combinators::collation;
use pg_sink_combinators::column;
use pg_sink_combinators::conversion;
use pg_sink_combinators::database;
use pg_sink_combinators::event_trigger;
use pg_sink_combinators::extension;
use pg_sink_combinators::foreign;
use pg_sink_combinators::index;
use pg_sink_combinators::language;
use pg_sink_combinators::large_object;
use pg_sink_combinators::materialized_view;
use pg_sink_combinators::publication;
use pg_sink_combinators::role;
use pg_sink_combinators::schema;
use pg_sink_combinators::sequence;
use pg_sink_combinators::server;
use pg_sink_combinators::statistics;
use pg_sink_combinators::string_or_null;
use pg_sink_combinators::subscription;
use pg_sink_combinators::table;
use pg_sink_combinators::tablespace;
use pg_sink_combinators::text_search;
use pg_sink_combinators::view;
use pg_sink_combinators::Foreign;
use pg_sink_combinators::TextSearch;
use pg_type_combinators::simple_typename;
