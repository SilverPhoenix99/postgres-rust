impl Parser<'_> {
    pub(in crate::parser) fn alter_conversion_stmt(&mut self) -> ParseResult<RawStmt> {

        /*
            ALTER CONVERSION any_name OWNER TO RoleSpec
            ALTER CONVERSION any_name RENAME TO ColId
            ALTER CONVERSION any_name SET SCHEMA ColId
        */

        let conversion = self.any_name().required(fn_info!())?;

        let op = self.buffer.consume_kw(|kw| matches!(kw, Owner | Rename | Set))
            .required(fn_info!())?;

        let stmt = match op {
            Owner => {
                self.buffer.consume_kw_eq(To).required(fn_info!())?;
                let new_owner = self.role_spec().required(fn_info!())?;

                AlterOwnerStmt::new(
                    AlterOwnerTarget::Conversion(conversion),
                    new_owner,
                ).into()
            },
            Rename => {
                self.buffer.consume_kw_eq(To).required(fn_info!())?;
                let new_name = self.col_id().required(fn_info!())?;

                RenameStmt::new(
                    RenameTarget::Conversion(conversion),
                    new_name,
                ).into()
            },
            Set => {
                self.buffer.consume_kw_eq(Schema).required(fn_info!())?;
                let new_schema = self.col_id().required(fn_info!())?;

                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Conversion(conversion),
                    new_schema,
                ).into()
            },
            _ => unreachable!("ALTER CONVERSION command must be one of OWNER, RENAME, or SET")
        };

        Ok(stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec::SessionUser;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter_conversion_owner() {
        let source = "conversion_name owner to session_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_conversion_stmt();

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Conversion(vec!["conversion_name".into()]),
            SessionUser
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_alter_conversion_rename() {
        let source = "conversion_name rename to other_conversion";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_conversion_stmt();

        let expected = RenameStmt::new(
            RenameTarget::Conversion(vec!["conversion_name".into()]),
            "other_conversion".into(),
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_alter_conversion_schema() {
        let source = "conversion_name set schema some_schema";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_conversion_stmt();

        let expected = AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Conversion(vec!["conversion_name".into()]),
            "some_schema".into(),
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use crate::{
    lexer::Keyword::{Owner, Rename, Schema, Set, To},
    parser::{
        ast_node::{
            AlterObjectSchemaStmt,
            AlterObjectSchemaTarget,
            AlterOwnerStmt,
            AlterOwnerTarget,
            RawStmt,
            RenameStmt,
            RenameTarget
        },
        result::Required,
        ParseResult,
        Parser
    }
};
use postgres_basics::fn_info;
