pub(super) fn alter_language_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER (PROCEDURAL)? LANGUAGE ColId OWNER TO RoleSpec # AlterOwnerStmt
        ALTER (PROCEDURAL)? LANGUAGE ColId RENAME TO ColId # RenameStmt
    */

    or(
        Procedural.and(Language).skip(),
        Language.skip()
    )
        .and_right(col_id())
        .chain(match_first_with_state!(|name, stream| {
            {
                Owner.and(To)
                    .and_right(role_spec())
            } => (role) {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Language(name),
                    role
                ).into()
            },
            {
                Rename.and(To)
                    .and_right(col_id())
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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_parser_ast::RoleSpec::Public;

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

use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_spec;
use postgres_parser_ast::AlterOwnerStmt;
use postgres_parser_ast::AlterOwnerTarget;
use postgres_parser_ast::RawStmt;
use postgres_parser_ast::RenameStmt;
use postgres_parser_ast::RenameTarget;
use postgres_parser_lexer::Keyword::Language;
use postgres_parser_lexer::Keyword::Owner;
use postgres_parser_lexer::Keyword::Procedural;
use postgres_parser_lexer::Keyword::Rename;
use postgres_parser_lexer::Keyword::To;
