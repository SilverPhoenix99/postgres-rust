impl Parser<'_> {
    pub(in crate::parser) fn alter_large_object_stmt(&mut self) -> OptResult<AstNode> {

        // ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec

        if self.buffer.consume_kw_eq(Unreserved(Large))?.is_none() {
            return Ok(None)
        }

        self.buffer.consume_kw_eq(Unreserved(Object)).required()?;

        let oid = self.signed_number().required()?;

        self.buffer.consume_kw_eq(Unreserved(Owner)).required()?;
        self.buffer.consume_kw_eq(Reserved(To)).required()?;

        let new_owner = self.role_spec().required()?;

        Ok(Some(
            AlterOwnerStmt::new(
                AlterOwnerTarget::LargeObject(oid),
                new_owner
            ).into()
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast_node::{AlterOwnerStmt, AlterOwnerTarget, RoleSpec, SignedNumber};
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::Parser;

    #[test]
    fn test_alter_large_object() {
        let source = "large object +654987 owner to some_user";
        let mut parser = Parser::new(source, DEFAULT_CONFIG);

        let actual = parser.alter_large_object_stmt();

        assert_matches!(actual, Ok(Some(_)));
        let actual = actual.unwrap().unwrap();

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::LargeObject(SignedNumber::SignedIConst(654987)),
            RoleSpec::Name("some_user".into())
        );

        assert_eq!(actual, expected.into());
    }
}

use crate::lexer::Keyword::{Reserved, Unreserved};
use crate::lexer::ReservedKeyword::To;
use crate::lexer::UnreservedKeyword::{Large, Object, Owner};
use crate::parser::ast_node::{AlterOwnerStmt, AlterOwnerTarget, AstNode};
use crate::parser::result::{OptResult, OptionalResult};
use crate::parser::Parser;
