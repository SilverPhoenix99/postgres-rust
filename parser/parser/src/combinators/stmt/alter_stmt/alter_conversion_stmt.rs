pub(super) fn alter_conversion_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER CONVERSION any_name OWNER TO RoleSpec
        ALTER CONVERSION any_name RENAME TO ColId
        ALTER CONVERSION any_name SET SCHEMA ColId
    */

    Conversion
        .and_right(any_name())
        .chain(match_first_with_state!{|conversion, stream| {
            {
                Owner.and(To)
                    .and_right(role_spec())
            } => (new_owner) {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Conversion(conversion),
                    new_owner,
                ).into()
            },
            {
                Rename.and(To)
                    .and_right(col_id())
            } => (new_name) {
                RenameStmt::new(
                    RenameTarget::Conversion(conversion),
                    new_name,
                ).into()
            },
            {
                Set.and(Schema)
                    .and_right(col_id())
            } => (new_schema) {
                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Conversion(conversion),
                    new_schema,
                ).into()
            }
        }})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use pg_ast::RoleSpec::SessionUser;

    #[test]
    fn test_alter_conversion_owner() {
        let source = "conversion conversion_name owner to session_user";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = alter_conversion_stmt().parse(&mut stream);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Conversion(vec!["conversion_name".into()]),
            SessionUser
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_alter_conversion_rename() {
        let source = "conversion conversion_name rename to other_conversion";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = alter_conversion_stmt().parse(&mut stream);

        let expected = RenameStmt::new(
            RenameTarget::Conversion(vec!["conversion_name".into()]),
            "other_conversion"
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_alter_conversion_schema() {
        let source = "conversion conversion_name set schema some_schema";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = alter_conversion_stmt().parse(&mut stream);

        let expected = AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Conversion(vec!["conversion_name".into()]),
            "some_schema"
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::match_first_with_state;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_spec;
use pg_ast::AlterObjectSchemaStmt;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_lexer::Keyword::Conversion;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
