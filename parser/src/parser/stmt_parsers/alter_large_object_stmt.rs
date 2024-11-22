pub(in crate::parser) fn alter_large_object_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER LARGE_P OBJECT_P NumericOnly OWNER TO RoleSpec
    */

    let parser = enclosure!(
        keyword(Large)
            .and(keyword(Object))
            .and_right(signed_number())
            .and_left(keyword(Owner))
            .and_left(keyword(To))
    );

    parser.and_then(role_spec(), |oid, new_owner|
        AlterOwnerStmt::new(
            AlterOwnerTarget::LargeObject(oid),
            new_owner
        ).into()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RoleSpec;
    use crate::parser::ast_node::SignedNumber;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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

use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::{Large, Object};
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::{enclosure, keyword, Combinator, CombinatorHelpers};
use crate::parser::const_numeric_parsers::signed_number;
use crate::parser::role_parsers::role_spec;
