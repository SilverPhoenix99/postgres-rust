impl Parser<'_> {

    /// Alias: `AlterGroupStmt`
    pub(in crate::parser) fn alter_group_stmt(&mut self) -> ParseResult<RawStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::alter_group_stmt";

        /*
            ALTER GROUP role_id RENAME TO role_id
            ALTER GROUP role_spec (ADD | DROP) USER role_list
        */

        let role_loc = self.buffer.current_location();
        let role = self.role_spec().required(fn_info!(FN_NAME))?;

        let action = self.buffer.consume_kw(|kw| matches!(kw, Add | DropKw | Rename))
            .required(fn_info!(FN_NAME))?;

        if action == Rename {
            return self.rename_group(role, role_loc).map(From::from);
        }

        /*
            ... (ADD | DROP) USER role_list
        */

        self.buffer.consume_kw_eq(User).required(fn_info!(FN_NAME))?;

        let action = if action == Add { AlterRoleAction::Add } else { AlterRoleAction::Remove };

        let roles = self.role_list().required(fn_info!(FN_NAME))?;
        let options = vec![RoleMembers(roles)];

        let stmt = AlterRoleStmt::new(role, action, options);
        Ok(stmt.into())
    }

    fn rename_group(&mut self, role: RoleSpec, role_loc: Location) -> ParseResult<RenameStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::rename_group";

        /*
            role_id RENAME TO role_id
        */

        let target = role.into_role_id(role_loc.clone())?;

        self.buffer.consume_kw_eq(To).required(fn_info!(FN_NAME))?;

        let new_name = self.role_spec()
            .required(fn_info!(FN_NAME))?
            .into_role_id(role_loc)?;

        let stmt = RenameStmt::new(Role(target), new_name);
        Ok(stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_group_rename() {
        let source = "some_group rename to new_group_name";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            Role("some_group".into()),
            "new_group_name".into()
        );

        assert_eq!(Ok(expected.into()), parser.alter_group_stmt());
    }

    #[test]
    fn test_add_role_to_group() {
        let source = "some_group add user current_role, new_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AlterRoleAction::Add,
            vec![RoleMembers(vec![
                RoleSpec::CurrentRole,
                RoleSpec::Name("new_user".into())
            ])]
        );

        assert_eq!(Ok(expected.into()), parser.alter_group_stmt());
    }

    #[test]
    fn test_drop_role_from_group() {
        let source = "some_group drop user session_user, public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = AlterRoleStmt::new(
            RoleSpec::Name("some_group".into()),
            AlterRoleAction::Remove,
            vec![RoleMembers(vec![
                RoleSpec::SessionUser,
                RoleSpec::Public
            ])]
        );

        assert_eq!(Ok(expected.into()), parser.alter_group_stmt());
    }
}

use crate::{
    lexer::Keyword::{Add, DropKw, Rename, To, User},
    parser::{
        ast_node::{
            AlterRoleAction,
            AlterRoleOption::RoleMembers,
            AlterRoleStmt,
            RawStmt,
            RenameStmt,
            RenameTarget::Role,
            RoleSpec
        },
        result::Required,
        ParseResult,
        Parser
    }
};
use postgres_basics::{fn_info, Location};
