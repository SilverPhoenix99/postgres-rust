impl Parser<'_> {
    pub(in crate::parser) fn alter_language_stmt(&mut self) -> ParseResult<RawStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::alter_language_stmt";

        /*
            ALTER (PROCEDURAL)? LANGUAGE ColId OWNER TO RoleSpec # AlterOwnerStmt
            ALTER (PROCEDURAL)? LANGUAGE ColId RENAME TO ColId # RenameStmt
        */

        let name = self.col_id().required(fn_info!(FN_NAME))?;

        let action = self.buffer.consume_kws(|kw| matches!(kw, Owner | Rename))
            .required(fn_info!(FN_NAME))?;

        self.buffer.consume_kw_eq(To).required(fn_info!(FN_NAME))?;

        let stmt = if action == Owner {
            let role = self.role_spec().required(fn_info!(FN_NAME))?;
            AlterOwnerStmt::new(
                AlterOwnerTarget::Language(name),
                role
            ).into()
        }
        else {
            let new_name = self.col_id().required(fn_info!(FN_NAME))?;
            RenameStmt::new(
                RenameTarget::Language(name),
                new_name
            ).into()
        };

        Ok(stmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec::Public;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter_owner() {
        let source = "some_language owner to public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Language("some_language".into()),
            Public
        );

        assert_eq!(Ok(expected.into()), parser.alter_language_stmt());
    }

    #[test]
    fn test_rename() {
        let source = "some_language rename to new_lang";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            RenameTarget::Language("some_language".into()),
            "new_lang".into()
        );

        assert_eq!(Ok(expected.into()), parser.alter_language_stmt());
    }
}

use crate::{
    lexer::Keyword::{Owner, Rename, To},
    parser::{
        ast_node::{AlterOwnerStmt, AlterOwnerTarget, RawStmt, RenameStmt, RenameTarget},
        result::Required,
        ParseResult,
        Parser
    }
};
use postgres_basics::fn_info;
