pub(super) fn alter_large_object_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec
    */

    sequence!(
        Large.and(Object).skip(),
        signed_number(),
        Owner.and(To),
        role_spec
    ).map(|(_, oid, _, new_owner)|
        AlterOwnerStmt::new(
            AlterOwnerTarget::LargeObject(oid),
            new_owner
        ).into()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::RoleSpec;
    use pg_ast::SignedNumber;

    #[test]
    fn test_alter_large_object() {
        let source = "large object +654987 owner to some_user";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::LargeObject(SignedNumber::IntegerConst(654987)),
            RoleSpec::Name("some_user".into())
        );

        assert_eq!(Ok(expected.into()), alter_large_object_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_spec;
use crate::combinators::signed_number;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Large;
use pg_lexer::Keyword::Object;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::To;
