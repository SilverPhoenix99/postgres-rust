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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_parser_ast::RoleSpec::SessionUser;

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

use crate::parser::combinators::any_name;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_spec;
use postgres_parser_ast::AlterObjectSchemaStmt;
use postgres_parser_ast::AlterObjectSchemaTarget;
use postgres_parser_ast::AlterOwnerStmt;
use postgres_parser_ast::AlterOwnerTarget;
use postgres_parser_ast::RawStmt;
use postgres_parser_ast::RenameStmt;
use postgres_parser_ast::RenameTarget;
use postgres_parser_lexer::Keyword::Conversion;
use postgres_parser_lexer::Keyword::Owner;
use postgres_parser_lexer::Keyword::Rename;
use postgres_parser_lexer::Keyword::Schema;
use postgres_parser_lexer::Keyword::Set;
use postgres_parser_lexer::Keyword::To;
