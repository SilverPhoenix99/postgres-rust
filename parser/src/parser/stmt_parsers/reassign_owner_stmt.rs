impl Parser<'_> {
    pub(in crate::parser) fn reassign_owner_stmt(&mut self) -> OptResult<ReassignOwnedStmt> {

        if self.buffer.consume_kw_eq(Unreserved(Reassign))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Unreserved(OwnedKw)).required()?;
        self.buffer.consume_kw_eq(Unreserved(By)).required()?;

        let roles = self.role_list()?;

        self.buffer.consume_kw_eq(Reserved(To)).required()?;

        let new_role = self.role_spec().required()?;

        Ok(Some(ReassignOwnedStmt::new(roles, new_role)))
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::ReservedKeyword::To;
use crate::lexer::UnreservedKeyword::{By, OwnedKw, Reassign};
use crate::parser::result::OptionalResult;
use crate::parser::{OptResult, Parser, ReassignOwnedStmt};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::RoleSpec;

    #[test]
    fn test_reassign_owner_stmt() {
        let mut parser = Parser::new(b"reassign owned by public, test_role to target_role", DEFAULT_CONFIG);

        let expected = ReassignOwnedStmt::new(
            vec![RoleSpec::Public, RoleSpec::Name("test_role".into())],
            RoleSpec::Name("target_role".into())
        );

        assert_eq!(Ok(Some(expected)), parser.reassign_owner_stmt());
    }
}