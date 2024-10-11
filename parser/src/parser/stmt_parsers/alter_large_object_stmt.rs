impl Parser<'_> {
    pub(in crate::parser) fn alter_large_object_stmt(&mut self) -> ScanResult<RawStmt> {

        /*
            ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec
        */

        self.buffer.consume_kw_eq(Large)?;

        self.buffer.consume_kw_eq(Object).required()?;

        let oid = self.signed_number().required()?;

        self.buffer.consume_kw_eq(Owner).required()?;
        self.buffer.consume_kw_eq(To).required()?;

        let new_owner = self.role_spec().required()?;

        let stmt = AlterOwnerStmt::new(
            AlterOwnerTarget::LargeObject(oid),
            new_owner
        );

        Ok(stmt.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::{RoleSpec, SignedNumber};
    use crate::parser::tests::DEFAULT_CONFIG;

    #[test]
    fn test_alter_large_object() {
        let source = "large object +654987 owner to some_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::LargeObject(SignedNumber::SignedIConst(654987)),
            RoleSpec::Name("some_user".into())
        );

        assert_eq!(Ok(expected.into()), parser.alter_large_object_stmt());
    }
}

use crate::lexer::Keyword::{Large, Object, Owner, To};
use crate::parser::ast_node::{AlterOwnerStmt, AlterOwnerTarget, RawStmt};
use crate::parser::result::{ScanResult, ScanResultTrait};
use crate::parser::Parser;
