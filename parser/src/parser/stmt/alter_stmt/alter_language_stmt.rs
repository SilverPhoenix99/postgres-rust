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
    use crate::parser::ast_node::RoleSpec::Public;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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
            "new_lang".into()
        );

        assert_eq!(Ok(expected.into()), alter_language_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Language;
use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::Procedural;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::To;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::col_id;
use crate::parser::combinators::match_first_with_state;
use crate::parser::combinators::or;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::role_parsers::role_spec;
