impl Parser<'_> {
    /// Alias: `ReassignOwnedStmt`
    pub(in crate::parser) fn reassign_owned_stmt(&mut self) -> ParseResult<ReassignOwnedStmt> {
        const FN_NAME: &str = "postgres_parser::parser::Parser::reassign_owned_stmt";

        /*
            REASSIGN OWNED BY role_list TO RoleSpec
        */

        self.buffer.consume_kw_eq(OwnedKw).required(fn_info!(FN_NAME))?;
        self.buffer.consume_kw_eq(By).required(fn_info!(FN_NAME))?;

        let roles = self.role_list().required(fn_info!(FN_NAME))?;

        self.buffer.consume_kw_eq(To).required(fn_info!(FN_NAME))?;

        let new_role = self.role_spec().required(fn_info!(FN_NAME))?;

        Ok(ReassignOwnedStmt::new(roles, new_role))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_reassign_owner_stmt() {
        let mut parser = Parser::new("owned by public, test_role to target_role", DEFAULT_CONFIG);

        let expected = ReassignOwnedStmt::new(
            vec![RoleSpec::Public, RoleSpec::Name("test_role".into())],
            RoleSpec::Name("target_role".into())
        );

        assert_eq!(Ok(expected), parser.reassign_owned_stmt());
    }
}

use crate::lexer::Keyword::{By, OwnedKw, To};
use crate::parser::ast_node::ReassignOwnedStmt;
use crate::parser::result::Required;
use crate::parser::{ParseResult, Parser};
use postgres_basics::fn_info;
