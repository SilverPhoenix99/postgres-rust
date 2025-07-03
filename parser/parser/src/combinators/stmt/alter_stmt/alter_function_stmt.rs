enum Change {
    Extension {
        action: AddDrop,
        extension: Str,
    },
    Owner(RoleSpec),
    Name(Str),
    Schema(Str),
    Options(Vec<AlterFunctionOption>),
}

/// Alias: `AlterFunctionStmt`
pub(super) fn alter_function_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        ALTER (FUNCTION|PROCEDURE|ROUTINE) function_with_argtypes
        (
              ( NO )? DEPENDS ON EXTENSION ColId => AlterObjectDependsStmt
            | OWNER TO RoleSpec                  => AlterOwnerStmt
            | RENAME TO ColId                    => RenameStmt
            | SET SCHEMA ColId                   => AlterObjectSchemaStmt
            | SET SCHEMA SCONST ( RESTRICT )?    => AlterObjectSchemaStmt
            | alterfunc_opt_list ( RESTRICT )?   => AlterFunctionStmt
        )
    */

    // SET SCHEMA is inlined, because it conflicts with `alter_function_option -> SET set_rest_more`.

    let (func_type, signature, stmt) = (func_type, function_with_argtypes, changes).parse(stream)?;

    let stmt = match (func_type, stmt) {
        (AlterFunctionKind::Function, Change::Extension { action, extension }) => {
            let target = AlterObjectDependsTarget::Function(signature);
            AlterObjectDependsStmt::new(target, extension, action).into()
        },
        (AlterFunctionKind::Function, Change::Owner(new_owner)) => {
            let target = AlterOwnerTarget::Function(signature);
            AlterOwnerStmt::new(target, new_owner).into()
        },
        (AlterFunctionKind::Function, Change::Name(new_name)) => {
            let target = RenameTarget::Function(signature);
            RenameStmt::new(target, new_name).into()
        },
        (AlterFunctionKind::Function, Change::Schema(new_schema)) => {
            let target = AlterObjectSchemaTarget::Function(signature);
            AlterObjectSchemaStmt::new(target, new_schema).into()
        },

        (AlterFunctionKind::Procedure, Change::Extension { action, extension }) => {
            let target = AlterObjectDependsTarget::Procedure(signature);
            AlterObjectDependsStmt::new(target, extension, action).into()
        },
        (AlterFunctionKind::Procedure, Change::Owner(new_owner)) => {
            let target = AlterOwnerTarget::Procedure(signature);
            AlterOwnerStmt::new(target, new_owner).into()
        },
        (AlterFunctionKind::Procedure, Change::Name(new_name)) => {
            let target = RenameTarget::Procedure(signature);
            RenameStmt::new(target, new_name).into()
        },
        (AlterFunctionKind::Procedure, Change::Schema(new_schema)) => {
            let target = AlterObjectSchemaTarget::Procedure(signature);
            AlterObjectSchemaStmt::new(target, new_schema).into()
        },

        (AlterFunctionKind::Routine, Change::Extension { action, extension }) => {
            let target = AlterObjectDependsTarget::Routine(signature);
            AlterObjectDependsStmt::new(target, extension, action).into()
        },
        (AlterFunctionKind::Routine, Change::Owner(new_owner)) => {
            let target = AlterOwnerTarget::Routine(signature);
            AlterOwnerStmt::new(target, new_owner).into()
        },
        (AlterFunctionKind::Routine, Change::Name(new_name)) => {
            let target = RenameTarget::Routine(signature);
            RenameStmt::new(target, new_name).into()
        },
        (AlterFunctionKind::Routine, Change::Schema(new_schema)) => {
            let target = AlterObjectSchemaTarget::Routine(signature);
            AlterObjectSchemaStmt::new(target, new_schema).into()
        },

        (_, Change::Options(options)) => {
            AlterFunctionStmt::new(func_type, signature, options).into()
        },
    };

    Ok(stmt)
}

fn changes(stream: &mut TokenStream) -> scan::Result<Change> {
    or((
        change_extension,
        change_owner,
        rename,
        set_schema,
        options
    )).parse(stream)
}

fn change_extension(stream: &mut TokenStream) -> scan::Result<Change> {

    let (action, extension) = (
        or((
            (No, Depends, On, Extension).map(|_| AddDrop::Drop),
            (Depends, On, Extension).map(|_| AddDrop::Add)
        )),
        col_id
    ).parse(stream)?;

    Ok(Change::Extension { action, extension })
}

fn change_owner(stream: &mut TokenStream) -> scan::Result<Change> {

    let (.., new_owner) = (Owner, To, role_spec).parse(stream)?;
    Ok(Change::Owner(new_owner))
}

fn rename(stream: &mut TokenStream) -> scan::Result<Change> {

    let (.., new_name) = (Rename, To, col_id).parse(stream)?;
    Ok(Change::Name(new_name))
}

fn set_schema(stream: &mut TokenStream) -> scan::Result<Change> {

    let (.., new_schema) = (
        Set,
        Schema,
        or((
            col_id,
            (string, Restrict.optional())
                .map(|(new_schema, _)| new_schema.into())
        ))
    ).parse(stream)?;

    Ok(Change::Schema(new_schema))
}

fn options(stream: &mut TokenStream) -> scan::Result<Change> {

    let (options, _) = (alterfunc_opt_list, Restrict.optional())
        .parse(stream)?;

    Ok(Change::Options(options))
}

fn func_type(stream: &mut TokenStream) -> scan::Result<AlterFunctionKind> {

    or((
        Kw::Function.map(|_| AlterFunctionKind::Function),
        Kw::Procedure.map(|_| AlterFunctionKind::Procedure),
        Kw::Routine.map(|_| AlterFunctionKind::Routine),
    )).parse(stream)
}

fn alterfunc_opt_list(stream: &mut TokenStream) -> scan::Result<Vec<AlterFunctionOption>> {

    many(alter_function_option).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::{
        FunctionWithArgs,
        RoleSpec::CurrentUser,
        SetRestMore::ConfigurationParameter,
        ValueOrDefault,
    };
    use test_case::test_case;

    #[test_case(
        "function my_func() depends on extension my_extension",
        AlterObjectDependsStmt::new(
            AlterObjectDependsTarget::Function(
                FunctionWithArgs::new(
                    vec!["my_func".into()],
                    Some(None)
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
                    Some(None)
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
        test_parser!(source, alter_function_stmt, expected);
    }

    #[test_case("function", AlterFunctionKind::Function)]
    #[test_case("procedure", AlterFunctionKind::Procedure)]
    #[test_case("routine", AlterFunctionKind::Routine)]
    fn test_func_type(source: &str, expected: AlterFunctionKind) {
        test_parser!(source, func_type, expected);
    }

    #[test]
    fn test_alterfunc_opt_list() {
        test_parser!(
            source = "COST 100 LEAKPROOF true",
            parser = alterfunc_opt_list,
            expected = vec![
                AlterFunctionOption::Cost(100.into()),
                AlterFunctionOption::Leakproof(true)
            ]
    );
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::many;
use crate::combinators::foundation::or;
use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::function_with_argtypes;
use crate::combinators::role_spec;
use crate::combinators::stmt::alter_function_option;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::AddDrop;
use pg_ast::AlterFunctionKind;
use pg_ast::AlterFunctionOption;
use pg_ast::AlterFunctionStmt;
use pg_ast::AlterObjectDependsStmt;
use pg_ast::AlterObjectDependsTarget;
use pg_ast::AlterObjectSchemaStmt;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_ast::RoleSpec;
use pg_basics::Str;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::Depends;
use pg_lexer::Keyword::Extension;
use pg_lexer::Keyword::No;
use pg_lexer::Keyword::On;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Restrict;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
