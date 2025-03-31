pub(super) fn privilege_target() -> impl Combinator<Output = PrivilegeTarget> {

    /*
        ALL FUNCTIONS IN SCHEMA name_list
      | ALL PROCEDURES IN SCHEMA name_list
      | ALL ROUTINES IN SCHEMA name_list
      | ALL SEQUENCES IN SCHEMA name_list
      | ALL TABLES IN SCHEMA name_list
      | DATABASE name_list
      | DOMAIN any_name_list
      | FOREIGN DATA WRAPPER name_list
      | FOREIGN SERVER name_list
      | FUNCTION function_with_argtypes_list
      | LANGUAGE name_list
      | LARGE OBJECT NumericOnly_list
      | PARAMETER parameter_name_list
      | PROCEDURE function_with_argtypes_list
      | ROUTINE function_with_argtypes_list
      | SCHEMA name_list
      | SEQUENCE qualified_name_list
      | TABLESPACE name_list
      | TYPE any_name_list
      | ( TABLE )? qualified_name_list
    */

    match_first! {
        All.and_right(match_first! {
            sequence!(Functions, In, Kw::Schema)
                .and_right(name_list())
                .map(AllFunctionsInSchema),
            sequence!(Procedures, In, Kw::Schema)
                .and_right(name_list())
                .map(AllProceduresInSchema),
            sequence!(Routines, In, Kw::Schema)
                .and_right(name_list())
                .map(AllRoutinesInSchema),
            sequence!(Sequences, In, Kw::Schema)
                .and_right(name_list())
                .map(AllSequencesInSchema),
            sequence!(Tables, In, Kw::Schema)
                .and_right(name_list())
                .map(AllTablesInSchema),
        }),
        Kw::Database
            .and_right(name_list())
            .map(Database),
        Kw::Domain
            .and_right(any_name_list())
            .map(Domain),
        Foreign.and_right(match_first! {
            sequence!(Data, Wrapper)
                .and_right(name_list())
                .map(ForeignDataWrapper),
            Server
                .and_right(name_list())
                .map(ForeignServer),
        }),
        Kw::Function
            .and_right(function_with_argtypes_list())
            .map(Function),
        Kw::Language
            .and_right(name_list())
            .map(Language),
        sequence!(Large, Object)
            .and_right(signed_number_list())
            .map(LargeObject),
        Parameter
            .and_right(parameter_name_list())
            .map(ParameterAcl),
        Kw::Procedure
            .and_right(function_with_argtypes_list())
            .map(Procedure),
        Kw::Routine
            .and_right(function_with_argtypes_list())
            .map(Routine),
        Kw::Schema
            .and_right(name_list())
            .map(Schema),
        Kw::Sequence
            .and_right(qualified_name_list())
            .map(Sequence),
        Kw::Tablespace
            .and_right(name_list())
            .map(Tablespace),
        Kw::Type
            .and_right(any_name_list())
            .map(Type),
        Kw::Table.optional()
            .and_right(qualified_name_list())
            .map(Table)
    }
}

fn parameter_name_list() -> impl Combinator<Output = Vec<QualifiedName>> {

    /*
        parameter_name ( ',' parameter_name )*
    */

    many_sep(Comma, parameter_name())
}

fn parameter_name() -> impl Combinator<Output = QualifiedName> {

    /*
        ColId ( '.' ColId )*
    */

    many_sep(Dot, col_id())
}

/// Alias: `NumericOnly_list`
fn signed_number_list() -> impl Combinator<Output = Vec<SignedNumber>> {

    /*
        signed_number ( ',' signed_number )*
    */

    many_sep(Comma, signed_number())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::FunctionWithArgs;
    #[allow(unused_imports)]
    use crate::parser::ast_node::RelationName;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("all functions in schema a, b",
        AllFunctionsInSchema(vec!["a".into(), "b".into()])
    )]
    #[test_case("all procedures in schema a, b",
        AllProceduresInSchema(vec!["a".into(), "b".into()])
    )]
    #[test_case("all routines in schema a, b",
        AllRoutinesInSchema(vec!["a".into(), "b".into()])
    )]
    #[test_case("all sequences in schema a, b",
        AllSequencesInSchema(vec!["a".into(), "b".into()])
    )]
    #[test_case("all tables in schema a, b",
        AllTablesInSchema(vec!["a".into(), "b".into()])
    )]
    #[test_case("database a, b",
        Database(vec!["a".into(), "b".into()])
    )]
    #[test_case("domain a, b",
        Domain(vec![vec!["a".into()], vec!["b".into()]])
    )]
    #[test_case("foreign data wrapper a, b",
        ForeignDataWrapper(vec!["a".into(), "b".into()])
    )]
    #[test_case("foreign server a, b",
        ForeignServer(vec!["a".into(), "b".into()])
    )]
    #[test_case("function a, b",
        Function(vec![
            FunctionWithArgs::new(vec!["a".into()], None),
            FunctionWithArgs::new(vec!["b".into()], None)
        ])
    )]
    #[test_case("language a, b",
        Language(vec!["a".into(), "b".into()])
    )]
    #[test_case("large object 1, 2",
        LargeObject(vec![1.into(), 2.into()])
    )]
    #[test_case("parameter a, b",
        ParameterAcl(vec![
            vec!["a".into()],
            vec!["b".into()]
        ])
    )]
    #[test_case("procedure a, b",
        Procedure(vec![
            FunctionWithArgs::new(vec!["a".into()], None),
            FunctionWithArgs::new(vec!["b".into()], None)
        ])
    )]
    #[test_case("routine a, b",
        Routine(vec![
            FunctionWithArgs::new(vec!["a".into()], None),
            FunctionWithArgs::new(vec!["b".into()], None)
        ])
    )]
    #[test_case("schema a, b",
        Schema(vec!["a".into(), "b".into()])
    )]
    #[test_case("sequence a, b",
        Sequence(vec![
            RelationName::new("a", None),
            RelationName::new("b", None)
        ])
    )]
    #[test_case("tablespace a, b",
        Tablespace(vec!["a".into(), "b".into()])
    )]
    #[test_case("type a, b",
        Type(vec![
            vec!["a".into()],
            vec!["b".into()]
        ])
    )]
    #[test_case("table a, b",
        Table(vec![
            RelationName::new("a", None),
            RelationName::new("b", None)
        ])
    )]
    #[test_case("a, b",
        Table(vec![
            RelationName::new("a", None),
            RelationName::new("b", None)
        ])
    )]
    fn test_privilege_target(source: &str, expected: PrivilegeTarget) {
        test_parser!(source, privilege_target(), expected)
    }

    #[test]
    fn test_parameter_name_list() {
        test_parser!(
            source = "a.b.c, d.e.f",
            parser = parameter_name_list(),
            expected = vec![
                vec!["a".into(), "b".into(), "c".into()],
                vec!["d".into(), "e".into(), "f".into()]
            ]
        )
    }

    #[test]
    fn test_parameter_name() {
        test_parser!(
            source = "a.b.c",
            parser = parameter_name(),
            expected = vec!["a".into(), "b".into(), "c".into()]
        )
    }

    #[test]
    fn test_signed_number_list() {
        test_parser!(
            source = "1, 2, 3",
            parser = signed_number_list(),
            expected = vec![
                SignedNumber::IntegerConst(1),
                SignedNumber::IntegerConst(2),
                SignedNumber::IntegerConst(3)
            ]
        )
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::All;
use crate::lexer::Keyword::Data;
use crate::lexer::Keyword::Foreign;
use crate::lexer::Keyword::Functions;
use crate::lexer::Keyword::In;
use crate::lexer::Keyword::Large;
use crate::lexer::Keyword::Object;
use crate::lexer::Keyword::Parameter;
use crate::lexer::Keyword::Procedures;
use crate::lexer::Keyword::Routines;
use crate::lexer::Keyword::Sequences;
use crate::lexer::Keyword::Server;
use crate::lexer::Keyword::Tables;
use crate::lexer::Keyword::Wrapper;
use crate::lexer::OperatorKind::Comma;
use crate::lexer::OperatorKind::Dot;
use crate::parser::ast_node::PrivilegeTarget;
use crate::parser::ast_node::PrivilegeTarget::AllFunctionsInSchema;
use crate::parser::ast_node::PrivilegeTarget::AllProceduresInSchema;
use crate::parser::ast_node::PrivilegeTarget::AllRoutinesInSchema;
use crate::parser::ast_node::PrivilegeTarget::AllSequencesInSchema;
use crate::parser::ast_node::PrivilegeTarget::AllTablesInSchema;
use crate::parser::ast_node::PrivilegeTarget::Database;
use crate::parser::ast_node::PrivilegeTarget::Domain;
use crate::parser::ast_node::PrivilegeTarget::ForeignDataWrapper;
use crate::parser::ast_node::PrivilegeTarget::ForeignServer;
use crate::parser::ast_node::PrivilegeTarget::Function;
use crate::parser::ast_node::PrivilegeTarget::Language;
use crate::parser::ast_node::PrivilegeTarget::LargeObject;
use crate::parser::ast_node::PrivilegeTarget::ParameterAcl;
use crate::parser::ast_node::PrivilegeTarget::Procedure;
use crate::parser::ast_node::PrivilegeTarget::Routine;
use crate::parser::ast_node::PrivilegeTarget::Schema;
use crate::parser::ast_node::PrivilegeTarget::Sequence;
use crate::parser::ast_node::PrivilegeTarget::Table;
use crate::parser::ast_node::PrivilegeTarget::Tablespace;
use crate::parser::ast_node::PrivilegeTarget::Type;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::SignedNumber;
use crate::parser::combinators::any_name_list;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes_list;
use crate::parser::combinators::name_list;
use crate::parser::combinators::qualified_name_list;
use crate::parser::combinators::signed_number;
