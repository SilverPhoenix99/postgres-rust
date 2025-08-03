pub(super) fn privilege_target(stream: &mut TokenStream) -> scan::Result<PrivilegeTarget> {

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

    alt!(
        seq!(
            All,
            alt!(
                seq!(Functions, In, Kw::Schema, name_list)
                    .map(|(.., schemas)| AllFunctionsInSchema(schemas)),
                seq!(Procedures, In, Kw::Schema, name_list)
                    .map(|(.., schemas)| AllProceduresInSchema(schemas)),
                seq!(Routines, In, Kw::Schema, name_list)
                    .map(|(.., schemas)| AllRoutinesInSchema(schemas)),
                seq!(Sequences, In, Kw::Schema, name_list)
                    .map(|(.., schemas)| AllSequencesInSchema(schemas)),
                seq!(Tables, In, Kw::Schema, name_list)
                    .map(|(.., schemas)| AllTablesInSchema(schemas)),
            )
        )
            .map(|(_, target)| target),
        seq!(Kw::Database, name_list)
            .map(|(_, db_names)| Database(db_names)),
        seq!(Kw::Domain, any_name_list)
            .map(|(_, domains)| Domain(domains)),
        seq!(
            Foreign,
            alt!(
                seq!(Data, Wrapper, name_list)
                    .map(|(.., fdws)| ForeignDataWrapper(fdws)),
                seq!(Server, name_list)
                    .map(|(_, servers)| ForeignServer(servers)),
            )
        )
            .map(|(_, target)| target),
        seq!(Kw::Function, function_with_argtypes_list)
            .map(|(_, signatures)| Function(signatures)),
        seq!(Kw::Language, name_list)
            .map(|(_, languages)| Language(languages)),
        seq!(Large, Object, signed_number_list)
            .map(|(.., lob_ids)| LargeObject(lob_ids)),
        seq!(Parameter, parameter_name_list)
            .map(|(_, config_parameters)| ParameterAcl(config_parameters)),
        seq!(Kw::Procedure, function_with_argtypes_list)
            .map(|(_, signatures)| Procedure(signatures)),
        seq!(Kw::Routine, function_with_argtypes_list)
            .map(|(_, signatures)| Routine(signatures)),
        seq!(Kw::Schema, name_list)
            .map(|(_, schemas)| Schema(schemas)),
        seq!(Kw::Sequence, qualified_name_list)
            .map(|(_, sequences)| Sequence(sequences)),
        seq!(Kw::Tablespace, name_list)
            .map(|(_, tablespaces)| Tablespace(tablespaces)),
        seq!(Kw::Type, any_name_list)
            .map(|(_, types)| Type(types)),
        seq!(Kw::Table.optional(), qualified_name_list)
            .map(|(_, tables)| Table(tables))
    ).parse(stream)
}

fn parameter_name_list(stream: &mut TokenStream) -> scan::Result<Vec<QualifiedName>> {

    /*
        parameter_name ( ',' parameter_name )*
    */

    many_sep(Comma, parameter_name).parse(stream)
}

fn parameter_name(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        ColId ( '.' ColId )*
    */

    many_sep(Dot, col_id).parse(stream)
}

/// Alias: `NumericOnly_list`
fn signed_number_list(stream: &mut TokenStream) -> scan::Result<Vec<SignedNumber>> {

    /*
        signed_number ( ',' signed_number )*
    */

    many_sep(Comma, signed_number).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::FunctionWithArgs;
    #[allow(unused_imports)]
    use pg_ast::RelationName;
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
        test_parser!(source, privilege_target, expected)
    }

    #[test]
    fn test_parameter_name_list() {
        test_parser!(
            source = "a.b.c, d.e.f",
            parser = parameter_name_list,
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
            parser = parameter_name,
            expected = vec!["a".into(), "b".into(), "c".into()]
        )
    }

    #[test]
    fn test_signed_number_list() {
        test_parser!(
            source = "1, 2, 3",
            parser = signed_number_list,
            expected = vec![
                SignedNumber::IntegerConst(1),
                SignedNumber::IntegerConst(2),
                SignedNumber::IntegerConst(3)
            ]
        )
    }
}

use crate::combinators::any_name_list;
use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::function_with_argtypes_list;
use crate::combinators::name_list;
use crate::combinators::qualified_name_list;
use crate::combinators::signed_number;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::PrivilegeTarget;
use pg_ast::PrivilegeTarget::AllFunctionsInSchema;
use pg_ast::PrivilegeTarget::AllProceduresInSchema;
use pg_ast::PrivilegeTarget::AllRoutinesInSchema;
use pg_ast::PrivilegeTarget::AllSequencesInSchema;
use pg_ast::PrivilegeTarget::AllTablesInSchema;
use pg_ast::PrivilegeTarget::Database;
use pg_ast::PrivilegeTarget::Domain;
use pg_ast::PrivilegeTarget::ForeignDataWrapper;
use pg_ast::PrivilegeTarget::ForeignServer;
use pg_ast::PrivilegeTarget::Function;
use pg_ast::PrivilegeTarget::Language;
use pg_ast::PrivilegeTarget::LargeObject;
use pg_ast::PrivilegeTarget::ParameterAcl;
use pg_ast::PrivilegeTarget::Procedure;
use pg_ast::PrivilegeTarget::Routine;
use pg_ast::PrivilegeTarget::Schema;
use pg_ast::PrivilegeTarget::Sequence;
use pg_ast::PrivilegeTarget::Table;
use pg_ast::PrivilegeTarget::Tablespace;
use pg_ast::PrivilegeTarget::Type;
use pg_ast::SignedNumber;
use pg_basics::QualifiedName;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Data;
use pg_lexer::Keyword::Foreign;
use pg_lexer::Keyword::Functions;
use pg_lexer::Keyword::In;
use pg_lexer::Keyword::Large;
use pg_lexer::Keyword::Object;
use pg_lexer::Keyword::Parameter;
use pg_lexer::Keyword::Procedures;
use pg_lexer::Keyword::Routines;
use pg_lexer::Keyword::Sequences;
use pg_lexer::Keyword::Server;
use pg_lexer::Keyword::Tables;
use pg_lexer::Keyword::Wrapper;
use pg_lexer::OperatorKind::Comma;
use pg_lexer::OperatorKind::Dot;
