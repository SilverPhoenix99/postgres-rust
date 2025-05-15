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
    use crate::parser::tests::test_parser;
    #[allow(unused_imports)]
    use postgres_parser_ast::FunctionWithArgs;
    #[allow(unused_imports)]
    use postgres_parser_ast::RelationName;
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
use postgres_basics::QualifiedName;
use postgres_parser_ast::PrivilegeTarget;
use postgres_parser_ast::PrivilegeTarget::AllFunctionsInSchema;
use postgres_parser_ast::PrivilegeTarget::AllProceduresInSchema;
use postgres_parser_ast::PrivilegeTarget::AllRoutinesInSchema;
use postgres_parser_ast::PrivilegeTarget::AllSequencesInSchema;
use postgres_parser_ast::PrivilegeTarget::AllTablesInSchema;
use postgres_parser_ast::PrivilegeTarget::Database;
use postgres_parser_ast::PrivilegeTarget::Domain;
use postgres_parser_ast::PrivilegeTarget::ForeignDataWrapper;
use postgres_parser_ast::PrivilegeTarget::ForeignServer;
use postgres_parser_ast::PrivilegeTarget::Function;
use postgres_parser_ast::PrivilegeTarget::Language;
use postgres_parser_ast::PrivilegeTarget::LargeObject;
use postgres_parser_ast::PrivilegeTarget::ParameterAcl;
use postgres_parser_ast::PrivilegeTarget::Procedure;
use postgres_parser_ast::PrivilegeTarget::Routine;
use postgres_parser_ast::PrivilegeTarget::Schema;
use postgres_parser_ast::PrivilegeTarget::Sequence;
use postgres_parser_ast::PrivilegeTarget::Table;
use postgres_parser_ast::PrivilegeTarget::Tablespace;
use postgres_parser_ast::PrivilegeTarget::Type;
use postgres_parser_ast::SignedNumber;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::All;
use postgres_parser_lexer::Keyword::Data;
use postgres_parser_lexer::Keyword::Foreign;
use postgres_parser_lexer::Keyword::Functions;
use postgres_parser_lexer::Keyword::In;
use postgres_parser_lexer::Keyword::Large;
use postgres_parser_lexer::Keyword::Object;
use postgres_parser_lexer::Keyword::Parameter;
use postgres_parser_lexer::Keyword::Procedures;
use postgres_parser_lexer::Keyword::Routines;
use postgres_parser_lexer::Keyword::Sequences;
use postgres_parser_lexer::Keyword::Server;
use postgres_parser_lexer::Keyword::Tables;
use postgres_parser_lexer::Keyword::Wrapper;
use postgres_parser_lexer::OperatorKind::Comma;
use postgres_parser_lexer::OperatorKind::Dot;
