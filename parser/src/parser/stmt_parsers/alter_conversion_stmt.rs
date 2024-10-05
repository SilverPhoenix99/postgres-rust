impl Parser<'_> {
    pub(in crate::parser) fn alter_conversion_stmt(&mut self) -> OptResult<AstNode> {
        /*
            ALTER CONVERSION_P any_name OWNER TO RoleSpec
            ALTER CONVERSION_P any_name RENAME TO ColId
            ALTER CONVERSION_P any_name SET SCHEMA ColId
        */

        if self.buffer.consume_kw_eq(Unreserved(Conversion))?.is_none() {
            return Ok(None)
        }

        let conversion = self.any_name().required()?;

        let op = self.buffer.consume(|tok|
            tok.keyword().and_then(KeywordDetails::unreserved)
                .filter(|kw|
                    matches!(kw, Owner | Rename | Set)
                )
        ).required()?;

        let stmt = match op {
            Owner => {
                self.buffer.consume_kw_eq(Reserved(To)).required()?;
                let new_owner = self.role_spec().required()?;

                AlterOwnerStmt::new(
                    AlterOwnerTarget::Conversion(conversion),
                    new_owner,
                ).into()
            },
            Rename => {
                self.buffer.consume_kw_eq(Reserved(To)).required()?;
                let new_name = self.col_id().required()?;

                RenameStmt::new(
                    RenameTarget::Conversion(conversion),
                    new_name,
                ).into()
            },
            Set => {
                self.buffer.consume_kw_eq(Unreserved(Schema)).required()?;
                let new_schema = self.col_id().required()?;

                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Conversion(conversion),
                    new_schema,
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
    use crate::parser::ast_node::RoleSpec::SessionUser;
    use crate::parser::ast_node::{AlterObjectSchemaStmt, AlterObjectSchemaTarget, AlterOwnerStmt, AlterOwnerTarget, RenameStmt, RenameTarget};
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter_conversion_owner() {
        let source = "conversion conversion_name owner to session_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_conversion_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Conversion(vec!["conversion_name".into()]),
            SessionUser
        );

        assert_eq!(actual, expected.into());
    }

    #[test]
    fn test_alter_conversion_rename() {
        let source = "conversion conversion_name rename to other_conversion";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_conversion_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = RenameStmt::new(
            RenameTarget::Conversion(vec!["conversion_name".into()]),
            "other_conversion".into(),
        );

        assert_eq!(actual, expected.into());
    }

    #[test]
    fn test_alter_conversion_schema() {
        let source = "conversion conversion_name set schema some_schema";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_conversion_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Conversion(vec!["conversion_name".into()]),
            "some_schema".into(),
        );

        assert_eq!(actual, expected.into());
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::KeywordDetails;
use crate::lexer::ReservedKeyword::To;
use crate::lexer::UnreservedKeyword::{Conversion, Owner, Rename, Schema, Set};
use crate::parser::ast_node::{AlterObjectSchemaTarget, AlterOwnerStmt, AlterOwnerTarget, AstNode, RenameStmt, RenameTarget};
use crate::parser::result::{OptResult, OptionalResult};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{ast_node, Parser};
use ast_node::AlterObjectSchemaStmt;
