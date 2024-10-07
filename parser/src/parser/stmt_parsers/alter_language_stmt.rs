impl Parser<'_> {
    pub(in crate::parser) fn alter_language_stmt(&mut self) -> OptResult<AstNode> {

        /*
            ALTER (PROCEDURAL)? LANGUAGE ColId OWNER TO RoleSpec # AlterOwnerStmt
            ALTER (PROCEDURAL)? LANGUAGE ColId RENAME TO ColId # RenameStmt
        */

        if self.buffer.consume_kw_eq(Procedural)?.is_some() {
            self.buffer.consume_kw_eq(Language).required()?;
        }
        else if self.buffer.consume_kw_eq(Language)?.is_none() {
            return Ok(None)
        }

        let name = self.col_id().required()?;

        let action = self.buffer.consume(|tok|
            tok.keyword().map(KeywordDetails::keyword)
                .filter(|kw|
                    matches!(kw, Owner | Rename)
                )
        ).required()?;

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

        Ok(Some(stmt))
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

        let actual = parser.alter_language_stmt();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Language("some_language".into()),
            Public
        );

        assert_eq!(AstNode::AlterOwnerStmt(expected), actual);
    }

    #[test]
    fn test_rename() {
        let source = "language some_language rename to new_lang";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_language_stmt();
        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = RenameStmt::new(
            RenameTarget::Language("some_language".into()),
            "new_lang".into()
        );

        assert_eq!(AstNode::RenameStmt(expected), actual);
    }
}

use crate::lexer::Keyword::{Language, Owner, Procedural, Rename, To};
use crate::lexer::KeywordDetails;
use crate::parser::ast_node::{AlterOwnerStmt, AlterOwnerTarget, AstNode, RenameStmt, RenameTarget};
use crate::parser::result::{OptResult, OptionalResult};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::Parser;
