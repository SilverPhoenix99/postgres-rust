/// Alias: `AlterFunctionStmt`
pub(super) fn alter_function_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER (FUNCTION|PROCEDURE|ROUTINE) function_with_argtypes
        (
              opt_no DEPENDS ON EXTENSION ColId => AlterObjectDependsStmt
            | OWNER TO RoleSpec                 => AlterOwnerStmt
            | RENAME TO ColId                   => RenameStmt
            | SET SCHEMA ColId                  => AlterObjectSchemaStmt
            | SET SCHEMA SCONST opt_restrict    => AlterObjectSchemaStmt
            | alterfunc_opt_list opt_restrict   => AlterFunctionStmt
        )
    */

    // SET SCHEMA is inlined, because it conflicts with `alter_function_option -> SET set_rest_more`.

    sequence!(
        func_type(),
        function_with_argtypes()
    )
        .chain(match_first_with_state!{|(func_type, func_sig), stream| {
            {
                sequence!(Depends, On, Extension)
                    .and_right(col_id())
            } => (extension) {
                let target = match func_type {
                    AlterFunctionKind::Function => AlterObjectDependsTarget::Function(func_sig),
                    AlterFunctionKind::Procedure => AlterObjectDependsTarget::Procedure(func_sig),
                    AlterFunctionKind::Routine => AlterObjectDependsTarget::Routine(func_sig),
                };
                AlterObjectDependsStmt::new(target, extension, AddDrop::Add).into()
            },
            {
                sequence!(No, Depends, On, Extension)
                    .and_right(col_id())
            } => (extension) {
                let target = match func_type {
                    AlterFunctionKind::Function => AlterObjectDependsTarget::Function(func_sig),
                    AlterFunctionKind::Procedure => AlterObjectDependsTarget::Procedure(func_sig),
                    AlterFunctionKind::Routine => AlterObjectDependsTarget::Routine(func_sig),
                };
                AlterObjectDependsStmt::new(target, extension, AddDrop::Drop).into()
            },
            {
                sequence!(Owner, To)
                    .and_right(role_spec())
            } => (new_owner) {
                let target = match func_type {
                    AlterFunctionKind::Function => AlterOwnerTarget::Function(func_sig),
                    AlterFunctionKind::Procedure => AlterOwnerTarget::Procedure(func_sig),
                    AlterFunctionKind::Routine => AlterOwnerTarget::Routine(func_sig),
                };
                AlterOwnerStmt::new(target, new_owner).into()
            },
            {
                sequence!(Rename, To)
                    .and_right(col_id())
            } => (new_name) {
                let target = match func_type {
                    AlterFunctionKind::Function => RenameTarget::Function(func_sig),
                    AlterFunctionKind::Procedure => RenameTarget::Procedure(func_sig),
                    AlterFunctionKind::Routine => RenameTarget::Routine(func_sig),
                };
                RenameStmt::new(target, new_name).into()
            },
            {
                sequence!(Set, Schema)
                    .and_right(or(
                        col_id(),
                        string()
                            .map(From::from)
                            .and_left(Restrict.optional())
                    ))
            } => (new_schema) {
                let target = match func_type {
                    AlterFunctionKind::Function => AlterObjectSchemaTarget::Function(func_sig),
                    AlterFunctionKind::Procedure => AlterObjectSchemaTarget::Procedure(func_sig),
                    AlterFunctionKind::Routine => AlterObjectSchemaTarget::Routine(func_sig),
                };
                AlterObjectSchemaStmt::new(target, new_schema).into()
            },
            {
                alterfunc_opt_list()
                    .and_left(Restrict.optional())
            } => (options) {
                AlterFunctionStmt::new(func_type, func_sig, options).into()
            }
        }})
}

fn func_type() -> impl Combinator<Output = AlterFunctionKind> {

    match_first! {
        Kw::Function.map(|_| AlterFunctionKind::Function),
        Kw::Procedure.map(|_| AlterFunctionKind::Procedure),
        Kw::Routine.map(|_| AlterFunctionKind::Routine),
    }
}

fn alterfunc_opt_list() -> impl Combinator<Output = Vec<AlterFunctionOption>> {

    many(alter_function_option())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::parser::ast_node::{
        FunctionWithArgs,
        RoleSpec::CurrentUser,
        SetRestMore::ConfigurationParameter,
        ValueOrDefault,
    };
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case(
        "function my_func() depends on extension my_extension",
        AlterObjectDependsStmt::new(
            AlterObjectDependsTarget::Function(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    Some(vec![])
                )
            ),
            "my_extension",
            AddDrop::Add
        ).into()
    )]
    #[test_case(
        "procedure my_func() no depends on extension my_extension",
        AlterObjectDependsStmt::new(
            AlterObjectDependsTarget::Procedure(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    Some(vec![])
                )
            ),
            "my_extension",
            AddDrop::Drop
        ).into()
    )]
    #[test_case(
        "routine my_func owner to current_user",
        AlterOwnerStmt::new(
            AlterOwnerTarget::Routine(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    None
                )
            ),
            CurrentUser
        ).into()
    )]
    #[test_case(
        "function my_func rename to new_name",
        RenameStmt::new(
            RenameTarget::Function(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    None
                )
            ),
            "new_name"
        ).into()
    )]
    #[test_case(
        "procedure my_func set schema new_schema",
        AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Procedure(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    None
                )
            ),
            "new_schema"
        ).into()
    )]
    #[test_case(
        "routine my_func set schema 'new_schema' restrict",
        AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Routine(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    None
                )
            ),
            "new_schema"
        ).into()
    )]
    #[test_case(
        "function my_func set schema 'new_schema'",
        AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Function(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    None
                )
            ),
            "new_schema"
        ).into()
    )]
    #[test_case(
        "procedure my_func leakproof cost 100 restrict",
        AlterFunctionStmt::new(
            AlterFunctionKind::Procedure,
            FunctionWithArgs::new(
                vec!["my_func".into()],
                None
            ),
            vec![
                AlterFunctionOption::Leakproof(true),
                AlterFunctionOption::Cost(100.into()),
            ]
        ).into()
    )]
    #[test_case(
        "routine my_func cost 100 not leakproof set foo='bar'",
        AlterFunctionStmt::new(
            AlterFunctionKind::Routine,
            FunctionWithArgs::new(
                vec!["my_func".into()],
                None
            ),
            vec![
                AlterFunctionOption::Cost(100.into()),
                AlterFunctionOption::Leakproof(false),
                AlterFunctionOption::Set(ConfigurationParameter {
                    name: vec!["foo".into()],
                    value: ValueOrDefault::Value(vec!["bar".into()])
                })
            ]
        ).into()
    )]
    fn test_alter_function_stmt(source: &str, expected: RawStmt) {
        test_parser!(source, alter_function_stmt(), expected);
    }

    #[test_case("function", AlterFunctionKind::Function)]
    #[test_case("procedure", AlterFunctionKind::Procedure)]
    #[test_case("routine", AlterFunctionKind::Routine)]
    fn test_func_type(source: &str, expected: AlterFunctionKind) {
        test_parser!(source, func_type(), expected);
    }

    #[test]
    fn test_alterfunc_opt_list() {
        test_parser!(
            source = "COST 100 LEAKPROOF true",
            parser = alterfunc_opt_list(),
            expected = vec![
                AlterFunctionOption::Cost(100.into()),
                AlterFunctionOption::Leakproof(true)
            ]
    );
    }
}

use crate::parser::ast_node::AddDrop;
use crate::parser::ast_node::AlterFunctionKind;
use crate::parser::ast_node::AlterFunctionOption;
use crate::parser::ast_node::AlterFunctionStmt;
use crate::parser::ast_node::AlterObjectDependsStmt;
use crate::parser::ast_node::AlterObjectDependsTarget;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterObjectSchemaTarget;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::string;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::function_with_argtypes;
use crate::parser::combinators::role_spec;
use crate::parser::combinators::stmt::alter_function_option;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::Depends;
use postgres_parser_lexer::Keyword::Extension;
use postgres_parser_lexer::Keyword::No;
use postgres_parser_lexer::Keyword::On;
use postgres_parser_lexer::Keyword::Owner;
use postgres_parser_lexer::Keyword::Rename;
use postgres_parser_lexer::Keyword::Restrict;
use postgres_parser_lexer::Keyword::Schema;
use postgres_parser_lexer::Keyword::Set;
use postgres_parser_lexer::Keyword::To;
