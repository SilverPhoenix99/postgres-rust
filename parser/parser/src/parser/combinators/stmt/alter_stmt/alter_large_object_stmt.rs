pub(super) fn alter_large_object_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec
    */

    sequence!(
        Large.and(Object).skip(),
        signed_number(),
        Owner.and(To),
        role_spec()
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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_parser_ast::RoleSpec;
    use postgres_parser_ast::SignedNumber;

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

use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_spec;
use crate::parser::combinators::signed_number;
use postgres_parser_ast::AlterOwnerStmt;
use postgres_parser_ast::AlterOwnerTarget;
use postgres_parser_ast::RawStmt;
use postgres_parser_lexer::Keyword::Large;
use postgres_parser_lexer::Keyword::Object;
use postgres_parser_lexer::Keyword::Owner;
use postgres_parser_lexer::Keyword::To;
