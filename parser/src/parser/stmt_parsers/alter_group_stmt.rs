impl Parser<'_> {

    /// Alias: `AlterGroupStmt`
    pub(in crate::parser) fn alter_group_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            ALTER GROUP role_id RENAME TO role_id
            ALTER GROUP role_spec (ADD | DROP) USER role_list
        */

        self.buffer.consume_kw_eq(Group)?;

        let role_loc = self.buffer.current_location();
        let role = self.role_spec().required()?;

        let action = self.buffer.consume_kws(|kw| matches!(kw, Add | DropKw | Rename))
            .required()?;

        if action == Rename {
            return self.rename_group(role, role_loc).map(RawStmt::from)
        }

        /*
            ... (ADD | DROP) USER role_list
        */

        self.buffer.consume_kw_eq(User).required()?;

        let action = if action == Add { AlterRoleAction::Add } else { AlterRoleAction::Remove };

        let roles = self.role_list().required()?;
        let options = vec![RoleMembers(roles)];

        let stmt = AlterRoleStmt::new(role, action, options);
        Ok(stmt.into())
    }

    fn rename_group(&mut self, role: RoleSpec, role_loc: Location) -> ScanResult<RenameStmt> {

        /*
            role_id RENAME TO role_id
        */

        let target = match role.into_role_id() {
            Ok(role_id) => role_id,
            Err(err) => {
                self.err_loc_override = Some(role_loc);
                return Err(err.into())
            }
        };

        self.buffer.consume_kw_eq(To).required()?;

        let new_name = self.role_spec().required()?.into_role_id()?;

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
        let source = "group some_group rename to new_group_name";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            Role("some_group".into()),
            "new_group_name".into()
        );

        assert_eq!(Ok(expected.into()), parser.alter_group_stmt());
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

        assert_eq!(Ok(expected.into()), parser.alter_group_stmt());
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

        assert_eq!(Ok(expected.into()), parser.alter_group_stmt());
    }
}

use crate::lexer::Keyword::{Add, DropKw, Group, Rename, To, User};
use crate::parser::ast_node::AlterRoleOption::RoleMembers;
use crate::parser::ast_node::RenameTarget::Role;
use crate::parser::ast_node::{AlterRoleAction, AlterRoleStmt, RawStmt, RenameStmt, RoleSpec};
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::Parser;
use postgres_basics::Location;
