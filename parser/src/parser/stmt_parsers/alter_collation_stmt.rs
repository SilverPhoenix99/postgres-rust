impl Parser<'_> {
    pub(in crate::parser) fn alter_collation_stmt(&mut self) -> ParseResult<RawStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::alter_collation_stmt";

        /*
            ALTER COLLATION any_name OWNER TO RoleSpec
            ALTER COLLATION any_name REFRESH VERSION_P
            ALTER COLLATION any_name RENAME TO ColId
            ALTER COLLATION any_name SET SCHEMA ColId
        */

        let name = self.any_name().required(fn_info!(FN_NAME))?;

        let op = self.buffer.consume_kws(|kw| matches!(kw, Owner | Refresh | Rename | Set))
            .required(fn_info!(FN_NAME))?;

        let stmt = match op {
            Owner => {
                self.buffer.consume_kw_eq(To).required(fn_info!(FN_NAME))?;
                let role = self.role_spec().required(fn_info!(FN_NAME))?;

                AlterOwnerStmt::new(
                    AlterOwnerTarget::Collation(name),
                    role
                ).into()
            },
            Refresh => {
                self.buffer.consume_kw_eq(Version).required(fn_info!(FN_NAME))?;
                RefreshCollationVersionStmt(name)
            },
            Rename => {
                self.buffer.consume_kw_eq(To).required(fn_info!(FN_NAME))?;
                let new_name = self.col_id().required(fn_info!(FN_NAME))?;

                RenameStmt::new(
                    RenameTarget::Collation(name),
                    new_name
                ).into()
            },
            Set => {
                self.buffer.consume_kw_eq(Schema).required(fn_info!(FN_NAME))?;
                let new_schema = self.col_id().required(fn_info!(FN_NAME))?;

                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Collation(name),
                    new_schema
                ).into()
            },
            _ => unreachable!("ALTER COLLATION command must be one of OWNER, REFRESH, RENAME, or SET")
        };

        Ok(stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec::CurrentUser;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_collation_owner() {
        let source = "collation_name owner to current_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Collation(vec!["collation_name".into()]),
            CurrentUser
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_collation_refresh_version() {
        let source = "collation_name refresh version";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();
        let expected = RefreshCollationVersionStmt(vec!["collation_name".into()]);
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_collation_rename() {
        let source = "collation_name rename to something_else";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();

        let expected = RenameStmt::new(
            RenameTarget::Collation(vec!["collation_name".into()]),
            "something_else".into()
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_collation_set_schema() {
        let source = "collation_name set schema some_schema";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();

        let expected = AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Collation(vec!["collation_name".into()]),
            "some_schema".into()
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use crate::{
    lexer::Keyword::{Owner, Refresh, Rename, Schema, Set, To, Version},
    parser::{
        ast_node::{
            AlterObjectSchemaStmt,
            AlterObjectSchemaTarget,
            AlterOwnerStmt,
            AlterOwnerTarget,
            RawStmt::{self, RefreshCollationVersionStmt},
            RenameStmt,
            RenameTarget
        },
        result::Required,
        ParseResult,
        Parser,
    },
};
use postgres_basics::fn_info;
