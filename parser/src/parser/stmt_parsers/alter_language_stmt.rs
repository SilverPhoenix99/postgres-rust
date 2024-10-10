impl Parser<'_> {
    pub(in crate::parser) fn alter_language_stmt(&mut self) -> ScanResult<AstNode> {

        /*
            ALTER (PROCEDURAL)? LANGUAGE ColId OWNER TO RoleSpec # AlterOwnerStmt
            ALTER (PROCEDURAL)? LANGUAGE ColId RENAME TO ColId # RenameStmt
        */

        let procedural = self.buffer.consume_kw_eq(Procedural).no_match_to_option()?.is_some();
        let language = self.buffer.consume_kw_eq(Language);

        if procedural {
            language.required()?;
        }
        else {
            language?;
        }

        let name = self.col_id().required()?;

        let action = self.buffer.consume_kws(|kw| matches!(kw, Owner | Rename))
            .required()?;

        self.buffer.consume_kw_eq(To).required()?;

        let stmt = if action == Owner {
            let role = self.role_spec().required()?;
            AlterOwnerStmt::new(
                AlterOwnerTarget::Language(name),
                role
            ).into()
        }
        else {
            let new_name = self.col_id().required()?;
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
        let source = "procedural language some_language owner to public";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Language("some_language".into()),
            Public
        );

        assert_eq!(Ok(expected.into()), parser.alter_language_stmt());
    }

    #[test]
    fn test_rename() {
        let source = "language some_language rename to new_lang";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            RenameTarget::Language("some_language".into()),
            "new_lang".into()
        );

        assert_eq!(Ok(expected.into()), parser.alter_language_stmt());
    }
}

use crate::lexer::Keyword::{Language, Owner, Procedural, Rename, To};
use crate::parser::ast_node::{AlterOwnerStmt, AlterOwnerTarget, AstNode, RenameStmt, RenameTarget};
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::Parser;
