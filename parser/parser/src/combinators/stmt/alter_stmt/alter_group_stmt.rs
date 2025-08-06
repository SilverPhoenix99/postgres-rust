enum Change {
    Rename(Str),
    Role {
        action: AddDrop,
        roles: Vec<RoleSpec>,
    }
}

/// Alias: `AlterGroupStmt`
pub(super) fn alter_group_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        ALTER GROUP role_id RENAME TO role_id
        ALTER GROUP role_spec (ADD | DROP) USER role_list
    */

    let (_, (group, loc), stmt) = seq!(
        Group,
        located!(role_spec),
        alt!(rename, change_role)
    ).parse(stream)?;

    let stmt = match stmt {
        Change::Rename(new_name) => {
            let group = group.into_role_id()
                .map_err(|err| err.at(loc))?;
            RenameStmt::new(Role(group), new_name).into()
        },
        Change::Role { action, roles } => {
            let options = Some(vec![RoleMembers(roles)]);
            AlterRoleStmt::new(group, action, options).into()
        }
    };

    Ok(stmt)
}

fn rename(stream: &mut TokenStream) -> scan::Result<Change> {

    let (.., new_name) = seq!(Rename, To, role_id).parse(stream)?;
    Ok(Change::Rename(new_name))
}

fn change_role(stream: &mut TokenStream) -> scan::Result<Change> {

    let (action, _, roles) = seq!(
        alt!(
            Add.map(|_| AddDrop::Add),
            DropKw.map(|_| AddDrop::Drop)
        ),
        User,
        role_list
    ).parse(stream)?;

    Ok(Change::Role { action, roles })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case(
        "group some_group rename to new_group_name",
        RenameStmt::new(
            Role("some_group".into()),
            "new_group_name"
        ).into()
    )]
    #[test_case(
        "group some_group add user current_role, new_user",
        AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AddDrop::Add,
            Some(vec![RoleMembers(vec![
                RoleSpec::CurrentRole,
                RoleSpec::Name("new_user".into())
            ])])
        ).into()
    )]
    #[test_case(
        "group some_group drop user session_user, public",
        AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AddDrop::Drop,
            Some(vec![RoleMembers(vec![
                RoleSpec::SessionUser,
                RoleSpec::Public
            ])])
        ).into()
    )]
    fn test_alter_group_stmt(source: &str, expected: RawStmt) {
        test_parser!(source, alter_group_stmt, expected)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::located;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_id;
use crate::combinators::role_list;
use crate::combinators::role_spec;
use pg_ast::AddDrop;
use pg_ast::AlterRoleOption::RoleMembers;
use pg_ast::AlterRoleStmt;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget::Role;
use pg_ast::RoleSpec;
use pg_basics::Str;
use pg_lexer::Keyword::Add;
use pg_lexer::Keyword::DropKw;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::User;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
