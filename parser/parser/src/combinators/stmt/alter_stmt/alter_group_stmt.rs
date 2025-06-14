/// Alias: `AlterGroupStmt`
pub(super) fn alter_group_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER GROUP role_id RENAME TO role_id
        ALTER GROUP role_spec (ADD | DROP) USER role_list
    */

    Group
        .and_right(located(role_spec()))
        .chain(match_first_with_state!{|(group, group_loc), stream| {
            {
                Rename.and(To)
                    .and_right(role_id())
            } => (new_name) {
                let group = group.into_role_id()
                    .map_err(|err|
                        LocatedError::new(err, group_loc)
                    )?;
                RenameStmt::new(Role(group), new_name).into()
            },
            {
                sequence!(
                    or(
                        Add.map(|_| AddDrop::Add),
                        DropKw.map(|_| AddDrop::Drop),
                    ),
                    User.skip(),
                    role_list()
                )
            } => ((action, _, roles)) {
                let options = Some(vec![RoleMembers(roles)]);
                AlterRoleStmt::new(group, action, options).into()
            }
        }})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::RoleSpec;

    #[test]
    fn test_group_rename() {
        let source = "group some_group rename to new_group_name";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            Role("some_group".into()),
            "new_group_name"
        );

        assert_eq!(Ok(expected.into()), alter_group_stmt().parse(&mut stream));
    }

    #[test]
    fn test_add_role_to_group() {
        let source = "group some_group add user current_role, new_user";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AddDrop::Add,
            Some(vec![RoleMembers(vec![
                RoleSpec::CurrentRole,
                RoleSpec::Name("new_user".into())
            ])])
        );

        assert_eq!(Ok(expected.into()), alter_group_stmt().parse(&mut stream));
    }

    #[test]
    fn test_drop_role_from_group() {
        let source = "group some_group drop user session_user, public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AddDrop::Drop,
            Some(vec![RoleMembers(vec![
                RoleSpec::SessionUser,
                RoleSpec::Public
            ])])
        );

        assert_eq!(Ok(expected.into()), alter_group_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::located;
use crate::combinators::foundation::match_first_with_state;
use crate::combinators::foundation::or;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_id;
use crate::combinators::role_list;
use crate::combinators::role_spec;
use pg_ast::AddDrop;
use pg_ast::AlterRoleOption::RoleMembers;
use pg_ast::AlterRoleStmt;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget::Role;
use pg_elog::LocatedError;
use pg_lexer::Keyword::Add;
use pg_lexer::Keyword::DropKw;
use pg_lexer::Keyword::Group;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::User;
