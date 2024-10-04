impl Parser<'_> {
    pub(in crate::parser) fn alter_collation_stmt(&mut self) -> OptResult<AstNode> {

        /*
            ALTER COLLATION any_name OWNER TO RoleSpec # AlterOwnerStmt
            ALTER COLLATION any_name REFRESH VERSION_P # AlterCollationStmt
            ALTER COLLATION any_name RENAME TO ColId # RenameStmt
            ALTER COLLATION any_name SET SCHEMA ColId # AlterObjectSchemaStmt
        */

        if self.buffer.consume_kw_eq(TypeFuncName(Collation))?.is_none() {
            return Ok(None)
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast_node::AlterOwnerStmt;
    use crate::parser::ast_node::AlterOwnerTarget::Collation;
    use crate::parser::ast_node::RoleSpec::CurrentUser;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_collation_owner() {
        let mut parser = Parser::new("alter collation collation_name owner to current_user", DEFAULT_CONFIG);
        let actual = parser.alter_collation_stmt();
        
        let expected = AlterOwnerStmt::new(
            Collation("collation_name".into()),
            CurrentUser
        );
        
        assert_eq!(actual, Ok(Some(expected.into())));
    }

    #[test]
    fn test_collation_rename() {
        todo!()
    }
}

use crate::lexer::Keyword::TypeFuncName;
use crate::lexer::TypeFuncNameKeyword::Collation;
use crate::parser::result::OptResult;
use crate::parser::AstNode;
use crate::parser::Parser;
