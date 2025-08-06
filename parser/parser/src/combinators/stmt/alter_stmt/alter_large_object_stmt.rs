pub(super) fn alter_large_object_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec
    */

    let (_, _, oid, _, _, new_owner) = seq!(Large, Object, signed_number, Owner, To, role_spec)
        .parse(stream)?;

    let stmt = AlterOwnerStmt::new(
        AlterOwnerTarget::LargeObject(oid),
        new_owner
    );

    Ok(stmt.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::RoleSpec;
    use pg_ast::SignedNumber;

    #[test]
    fn test_alter_large_object() {
        test_parser!(
            source = "large object +654987 owner to some_user",
            parser = alter_large_object_stmt,
            expected = AlterOwnerStmt::new(
                AlterOwnerTarget::LargeObject(SignedNumber::IntegerConst(654987)),
                RoleSpec::Name("some_user".into())
            )
        )
    }
}

use crate::combinators::foundation::seq;
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
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
