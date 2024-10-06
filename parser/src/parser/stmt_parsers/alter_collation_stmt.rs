impl Parser<'_> {
    pub(in crate::parser) fn alter_collation_stmt(&mut self) -> OptResult<AstNode> {

        /*
            ALTER COLLATION any_name OWNER TO RoleSpec
            ALTER COLLATION any_name REFRESH VERSION_P
            ALTER COLLATION any_name RENAME TO ColId
            ALTER COLLATION any_name SET SCHEMA ColId
        */

        if self.buffer.consume_kw_eq(Collation)?.is_none() {
            return Ok(None)
        }

        let name = self.any_name().required()?;

        let op = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw|
                    matches!(kw, Owner | Refresh | Rename | Set)
                )
        ).required()?;

        let stmt: AstNode = match op {
            Owner => {
                self.buffer.consume_kw_eq(To).required()?;
                let role = self.role_spec().required()?;
                AstNode::AlterOwnerStmt(
                    AlterOwnerStmt::new(
                        AlterOwnerTarget::Collation(name),
                        role
                    )
                )
            },
            Refresh => {
                self.buffer.consume_kw_eq(Version).required()?;
                RefreshCollationVersionStmt(name)
            },
            Rename => {
                self.buffer.consume_kw_eq(To).required()?;
                let new_name = self.col_id().required()?;

                RenameStmt::new(
                    RenameTarget::Collation(name),
                    new_name
                ).into()
            },
            Set => {
                self.buffer.consume_kw_eq(Schema).required()?;
                let new_schema = self.col_id().required()?;
                
                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Collation(name),
                    new_schema
                ).into()
            },
            _ => unreachable!()
        };

        Ok(Some(stmt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec::CurrentUser;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_collation_owner() {
        let source = "collation collation_name owner to current_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Collation(vec!["collation_name".into()]),
            CurrentUser
        );

        assert_eq!(actual, expected.into());
    }

    #[test]
    fn test_collation_refresh_version() {
        let source = "collation collation_name refresh version";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        assert_eq!(actual, RefreshCollationVersionStmt(vec!["collation_name".into()]));
    }

    #[test]
    fn test_collation_rename() {
        let source = "collation collation_name rename to something_else";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = RenameStmt::new(
            RenameTarget::Collation(vec!["collation_name".into()]),
            "something_else".into()
        );

        assert_eq!(actual, expected.into());
    }

    #[test]
    fn test_collation_set_schema() {
        let source = "collation collation_name set schema some_schema";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_collation_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Collation(vec!["collation_name".into()]),
            "some_schema".into()
        );

        assert_eq!(actual, expected.into());
    }
}

use crate::lexer::Keyword::{Collation, Owner, Refresh, Rename, Schema, Set, To, Version};
use crate::lexer::KeywordDetails;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterObjectSchemaTarget;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::AstNode::RefreshCollationVersionStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::result::{OptResult, OptionalResult};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::AstNode;
use crate::parser::Parser;
