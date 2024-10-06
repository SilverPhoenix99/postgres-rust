impl Parser<'_> {

    /// Alias: `AlterGroupStmt`
    pub(in crate::parser) fn alter_group_stmt(&mut self) -> OptResult<AstNode> {

        /*
            ALTER GROUP role_id RENAME TO role_id
            ALTER GROUP role_spec (ADD | DROP) USER role_list
        */

        if self.buffer.consume_kw_eq(Group)?.is_none() {
            return Ok(None)
        }

        let role_loc = self.buffer.current_location();
        let role = self.role_spec().required()?;

        let action = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw| matches!(kw, Add | DropKw | Rename))
        ).required()?;

        if action == Rename {
            let target = match role.into_role_id() {
                Ok(role_id) => role_id,
                Err(err) => {
                    self.err_loc_override = Some(role_loc);
                    return Err(Some(err))
                }
            };

            self.buffer.consume_kw_eq(To)?;
            let new_name = self.role_spec().required()?.into_role_id()?;

            return Ok(Some(
                RenameStmt::new(RenameTarget::Role(target), new_name).into()
            ))
        }

        self.buffer.consume_kw_eq(User)?;

        let action = if action == Add { AlterRoleAction::Add } else { AlterRoleAction::Remove };

        let roles = self.role_list()?;
        let options = vec![RoleMembers(roles)];

        Ok(Some(
            AlterRoleStmt::new(role, action, options)
                .into()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_group_rename() {
        let source = "group some_group rename to new_group_name";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            RenameTarget::Role("some_group".into()),
            "new_group_name".into()
        );

        assert_eq!(Ok(Some(expected.into())), parser.alter_group_stmt());
    }

    #[test]
    fn test_add_role_to_group() {
        let source = "group some_group add user current_role, new_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AlterRoleAction::Add,
            vec![RoleMembers(vec![
                RoleSpec::CurrentRole,
                RoleSpec::Name("new_user".into())
            ])]
        );

        assert_eq!(Ok(Some(expected.into())), parser.alter_group_stmt());
    }

    #[test]
    fn test_drop_role_from_group() {
        let source = "group some_group drop user session_user, public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AlterRoleAction::Remove,
            vec![RoleMembers(vec![
                RoleSpec::SessionUser,
                RoleSpec::Public
            ])]
        );

        assert_eq!(Ok(Some(expected.into())), parser.alter_group_stmt());
    }
}

use crate::lexer::Keyword::{Add, DropKw, Group, Rename, To, User};
use crate::lexer::KeywordDetails;
use crate::parser::ast_node::AlterRoleOption::RoleMembers;
use crate::parser::ast_node::{AlterRoleAction, AlterRoleStmt, AstNode, RenameStmt, RenameTarget, RoleSpec};
use crate::parser::result::{OptResult, OptionalResult};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::Parser;
