impl Parser<'_> {
    /// Alias: `ReassignOwnedStmt`
    pub(in crate::parser) fn reassign_owned_stmt(&mut self) -> OptResult<ReassignOwnedStmt> {

        /*
            REASSIGN OWNED BY role_list TO RoleSpec
        */

        if self.buffer.consume_kw_eq(Reassign)?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(OwnedKw).required()?;
        self.buffer.consume_kw_eq(By).required()?;

        let roles = self.role_list()?;

        self.buffer.consume_kw_eq(To).required()?;

        let new_role = self.role_spec().required()?;

        Ok(Some(ReassignOwnedStmt::new(roles, new_role)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_reassign_owner_stmt() {
        let mut parser = Parser::new("reassign owned by public, test_role to target_role", DEFAULT_CONFIG);

        let expected = ReassignOwnedStmt::new(
            vec![RoleSpec::Public, RoleSpec::Name("test_role".into())],
            RoleSpec::Name("target_role".into())
        );

        assert_eq!(Ok(Some(expected)), parser.reassign_owned_stmt());
    }
}

use crate::lexer::Keyword::{By, OwnedKw, Reassign, To};
use crate::parser::ast_node::ReassignOwnedStmt;
use crate::parser::result::OptionalResult;
use crate::parser::OptResult;
use crate::parser::Parser;
