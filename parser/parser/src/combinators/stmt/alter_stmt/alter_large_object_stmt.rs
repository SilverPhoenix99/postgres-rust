pub(super) fn alter_large_object_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec
    */

    let (_, _, oid, _, _, new_owner) = seq!(Large, Object, signed_number, Owner, To, role_spec)
        .parse(ctx)?;

    let stmt = AlterOwnerStmt::new(
        AlterOwnerTarget::LargeObject(oid),
        new_owner
    );

    Ok(stmt.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::RoleSpec;
    use pg_sink_ast::SignedNumber;

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

use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Large;
use pg_lexer::Keyword::Object;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_combinators::role_spec;
use pg_sink_combinators::signed_number;
