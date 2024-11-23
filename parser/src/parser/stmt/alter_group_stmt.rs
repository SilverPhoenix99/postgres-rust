/// Alias: `AlterGroupStmt`
pub(in crate::parser) fn alter_group_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER GROUP role_id RENAME TO role_id
        ALTER GROUP role_spec (ADD | DROP) USER role_list
    */

    keyword(Group)
        .and_right(located(role_spec()))
        .chain_result(match_first_with_state!{|(group, group_loc), stream| {
            {
                keyword(Rename)
                    .and(keyword(To))
                    .and_right(role_id())
            } => (new_name) {
                let group = group.into_role_id(group_loc)?;
                RenameStmt::new(Role(group), new_name).into()
            },
            {
                sequence!(
                    or(
                        keyword(Add).map(|_| AlterRoleAction::Add),
                        keyword(DropKw).map(|_| AlterRoleAction::Remove),
                    ),
                    keyword(User).skip(),
                    role_list()
                )
            } => ((action, _, roles)) {
                let options = vec![RoleMembers(roles)];
                AlterRoleStmt::new(group, action, options).into()
            }
        }})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_group_rename() {
        let source = "group some_group rename to new_group_name";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            Role("some_group".into()),
            "new_group_name".into()
        );

        assert_eq!(Ok(expected.into()), alter_group_stmt().parse(&mut stream));
    }

    #[test]
    fn test_add_role_to_group() {
        let source = "group some_group add user current_role, new_user";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AlterRoleAction::Add,
            vec![RoleMembers(vec![
                RoleSpec::CurrentRole,
                RoleSpec::Name("new_user".into())
            ])]
        );

        assert_eq!(Ok(expected.into()), alter_group_stmt().parse(&mut stream));
    }

    #[test]
    fn test_drop_role_from_group() {
        let source = "group some_group drop user session_user, public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AlterRoleAction::Remove,
            vec![RoleMembers(vec![
                RoleSpec::SessionUser,
                RoleSpec::Public
            ])]
        );

        assert_eq!(Ok(expected.into()), alter_group_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Add;
use crate::lexer::Keyword::DropKw;
use crate::lexer::Keyword::Group;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::User;
use crate::parser::ast_node::AlterRoleAction;
use crate::parser::ast_node::AlterRoleOption::RoleMembers;
use crate::parser::ast_node::AlterRoleStmt;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget::Role;
use crate::parser::combinators::keyword;
use crate::parser::combinators::match_first_with_state;
use crate::parser::combinators::or;
use crate::parser::combinators::sequence;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::located_combinator::located;
use crate::parser::role_parsers::role_id;
use crate::parser::role_parsers::role_list;
use crate::parser::role_parsers::role_spec;
