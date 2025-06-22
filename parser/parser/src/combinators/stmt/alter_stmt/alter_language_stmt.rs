pub(super) fn alter_language_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER (PROCEDURAL)? LANGUAGE ColId OWNER TO RoleSpec # AlterOwnerStmt
        ALTER (PROCEDURAL)? LANGUAGE ColId RENAME TO ColId # RenameStmt
    */

    or(
        Procedural.and(Language).skip(),
        Language.skip()
    )
        .and_right(parser(col_id))
        .chain(match_first_with_state!(|name, stream| {
            {
                Owner.and(To)
                    .and_right(parser(role_spec))
            } => (role) {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Language(name),
                    role
                ).into()
            },
            {
                Rename.and(To)
                    .and_right(parser(col_id))
            } => (new_name) {
                RenameStmt::new(
                    RenameTarget::Language(name),
                    new_name
                ).into()
            },
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::RoleSpec::Public;

    #[test]
    fn test_alter_owner() {
        let source = "procedural language some_language owner to public";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Language("some_language".into()),
            Public
        );

        assert_eq!(Ok(expected.into()), alter_language_stmt().parse(&mut stream));
    }

    #[test]
    fn test_rename() {
        let source = "language some_language rename to new_lang";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let expected = RenameStmt::new(
            RenameTarget::Language("some_language".into()),
            "new_lang"
        );

        assert_eq!(Ok(expected.into()), alter_language_stmt().parse(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::match_first_with_state;
use crate::combinators::foundation::or;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_spec;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_lexer::Keyword::Language;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Procedural;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::To;
