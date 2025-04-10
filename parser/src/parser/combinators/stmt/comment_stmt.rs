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
        access_method(),
        aggregate(),
        typecast(),
        collation(),
        column(),
        constraint(),
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
        operator(),
        language(),
        policy(),
        procedure(),
        publication(),
        role(),
        routine(),
        rule(),
        schema(),
        sequence(),
        foreign_server(),
        extended_statistics(),
        subscription(),
        table(),
        tablespace(),
        text_search_configuration(),
        transform(),
        trigger(),
        type_(),
        view()
    }
}

fn access_method() -> impl Combinator<Output = CommentTarget> {
    and(Access, Method)
        .and_right(col_id())
        .map(AccessMethod)
}

fn aggregate() -> impl Combinator<Output = CommentTarget> {
    Kw::Aggregate
        .and_right(aggregate_with_argtypes())
        .map(Aggregate)
}

fn typecast() -> impl Combinator<Output = CommentTarget> {
    Cast.and_right(between_paren(
        typename().and_then(
            As.and_right(typename()),
            |from_type, to_type| Typecast { from_type, to_type }
        )
    ))
}

fn collation() -> impl Combinator<Output = CommentTarget> {
    Kw::Collation
        .and_right(any_name())
        .map(Collation)
}

fn column() -> impl Combinator<Output = CommentTarget> {
    Kw::Column
        .and_right(any_name())
        .map(Column)
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

fn conversion() -> impl Combinator<Output = CommentTarget> {
    Kw::Conversion
        .and_right(any_name())
        .map(Conversion)
}

fn database() -> impl Combinator<Output = CommentTarget> {
    Kw::Database
        .and_right(col_id())
        .map(Database)
}

fn domain() -> impl Combinator<Output = CommentTarget> {
    Kw::Domain
        .and_right(typename())
        .map(Domain)
}

fn event_trigger() -> impl Combinator<Output = CommentTarget> {
    and(Event, Kw::Trigger)
        .and_right(col_id())
        .map(EventTrigger)
}

fn extension() -> impl Combinator<Output = CommentTarget> {
    Kw::Extension
        .and_right(col_id())
        .map(Extension)
}

fn foreign() -> impl Combinator<Output = CommentTarget> {
    Foreign.and_right(or(
        and(Data, Wrapper)
            .and_right(col_id())
            .map(ForeignDataWrapper),
        Kw::Table
            .and_right(any_name())
            .map(ForeignTable)
    ))
}

fn function() -> impl Combinator<Output = CommentTarget> {
    Kw::Function
        .and_right(function_with_argtypes())
        .map(Function)
}

fn index() -> impl Combinator<Output = CommentTarget> {
    Kw::Index
        .and_right(any_name())
        .map(Index)
}

fn large_object() -> impl Combinator<Output = CommentTarget> {
    and(Large, Object)
        .and_right(signed_number())
        .map(LargeObject)
}

fn materialized_view() -> impl Combinator<Output = CommentTarget> {
    and(Materialized, Kw::View)
        .and_right(any_name())
        .map(MaterializedView)
}

fn operator() -> impl Combinator<Output = CommentTarget> {
    Kw::Operator.and_right(match_first! {
        and(
            Class.and_right(any_name()),
            Using.and_right(col_id())
        ).map(|(name, index_method)| OperatorClass { name, index_method }),
        and(
            Family.and_right(any_name()),
            Using.and_right(col_id())
        ).map(|(name, index_method)| OperatorFamily { name, index_method }),
        operator_with_argtypes().map(Operator)
    })
}

fn language() -> impl Combinator<Output = CommentTarget> {
    or(
        Kw::Language.skip(),
        and(Procedural, Kw::Language).skip()
    )
        .and_right(col_id())
        .map(Language)
}

fn policy() -> impl Combinator<Output = CommentTarget> {
    Kw::Policy
        .and_right(col_id())
        .and_then(
            On.and_right(any_name()),
            |name, table| Policy { name, table }
        )
}

fn procedure() -> impl Combinator<Output = CommentTarget> {
    Kw::Procedure
        .and_right(function_with_argtypes())
        .map(Procedure)
}

fn publication() -> impl Combinator<Output = CommentTarget> {
    Kw::Publication
        .and_right(col_id())
        .map(Publication)
}

fn role() -> impl Combinator<Output = CommentTarget> {
    Kw::Role
        .and_right(col_id())
        .map(Role)
}

fn routine() -> impl Combinator<Output = CommentTarget> {
    Kw::Routine
        .and_right(function_with_argtypes())
        .map(Routine)
}

fn rule() -> impl Combinator<Output = CommentTarget> {
    Kw::Rule
        .and_right(col_id())
        .and_then(
            On.and_right(any_name()),
            |name, table| Rule { name, table }
        )
}

fn schema() -> impl Combinator<Output = CommentTarget> {
    Kw::Schema
        .and_right(col_id())
        .map(Schema)
}

fn sequence() -> impl Combinator<Output = CommentTarget> {
    Kw::Sequence
        .and_right(any_name())
        .map(Sequence)
}

fn foreign_server() -> impl Combinator<Output = CommentTarget> {
    Server
        .and_right(col_id())
        .map(ForeignServer)
}

fn extended_statistics() -> impl Combinator<Output = CommentTarget> {
    Statistics
        .and_right(any_name())
        .map(ExtendedStatistics)
}

fn subscription() -> impl Combinator<Output = CommentTarget> {
    Kw::Subscription
        .and_right(col_id())
        .map(Subscription)
}

fn table() -> impl Combinator<Output = CommentTarget> + Sized {
    Kw::Table
        .and_right(any_name())
        .map(Table)
}

fn tablespace() -> impl Combinator<Output = CommentTarget> + Sized {
    Kw::Tablespace
        .and_right(col_id())
        .map(Tablespace)
}

fn text_search_configuration() -> impl Combinator<Output = CommentTarget> {
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

fn transform() -> impl Combinator<Output = CommentTarget> {
    and(Kw::Transform, For)
        .and_right(typename())
        .and_then(
            Kw::Language.and_right(col_id()),
            |for_type, language| Transform { for_type, language }
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

fn type_() -> impl Combinator<Output = CommentTarget> {
    Kw::Type
        .and_right(typename())
        .map(Type)
}

fn view() -> impl Combinator<Output = CommentTarget> {
    Kw::View
        .and_right(any_name())
        .map(View)
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
        SignedNumber::IntegerConst,
        TypeName::{Int4, Varchar},
        Operator::Addition,
        QualifiedOperator,
        OperatorWithArgs,
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
        Typecast {
            from_type: Int4.into(),
            to_type: Varchar { max_length: None }.into()
        }
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
            Some(Int4.into()),
            Some(Int4.into())
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
    #[test_case("transform for int language some_language",
        Transform {
            for_type: Int4.into(),
            language: "some_language".into()
        }
    )]
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
use crate::lexer::Keyword::Access;
use crate::lexer::Keyword::As;
use crate::lexer::Keyword::Cast;
use crate::lexer::Keyword::Class;
use crate::lexer::Keyword::Comment;
use crate::lexer::Keyword::Configuration;
use crate::lexer::Keyword::Data;
use crate::lexer::Keyword::Dictionary;
use crate::lexer::Keyword::Event;
use crate::lexer::Keyword::Family;
use crate::lexer::Keyword::For;
use crate::lexer::Keyword::Foreign;
use crate::lexer::Keyword::Is;
use crate::lexer::Keyword::Large;
use crate::lexer::Keyword::Materialized;
use crate::lexer::Keyword::Method;
use crate::lexer::Keyword::Object;
use crate::lexer::Keyword::On;
use crate::lexer::Keyword::ParserKw;
use crate::lexer::Keyword::Procedural;
use crate::lexer::Keyword::Search;
use crate::lexer::Keyword::Server;
use crate::lexer::Keyword::Statistics;
use crate::lexer::Keyword::Template;
use crate::lexer::Keyword::Text;
use crate::lexer::Keyword::Using;
use crate::lexer::Keyword::Wrapper;
use crate::parser::ast_node::CommentStmt;
use crate::parser::ast_node::CommentTarget;
use crate::parser::ast_node::CommentTarget::*;
use crate::parser::combinators::any_name;
use crate::parser::combinators::between_paren;
use crate::parser::combinators::col_id;
use crate::parser::combinators::const_numeric::signed_number;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes;
use crate::parser::combinators::simple_typename;
use crate::parser::combinators::stmt::aggregate_with_argtypes;
use crate::parser::combinators::stmt::operator_with_argtypes;
use crate::parser::combinators::string_or_null;
use crate::parser::combinators::typename;
