pub(in crate::parser) fn alter_conversion_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER CONVERSION any_name OWNER TO RoleSpec
        ALTER CONVERSION any_name RENAME TO ColId
        ALTER CONVERSION any_name SET SCHEMA ColId
    */

    keyword(Conversion)
        .and_right(any_name())
        .chain_result(match_first_with_state!{|conversion, stream| {
            {
                keyword(Owner)
                    .and(keyword(To))
                    .and_right(role_spec())
            } => (new_owner) {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Conversion(conversion),
                    new_owner,
                ).into()
            },
            {
                keyword(Rename)
                    .and(keyword(To))
                    .and_right(col_id())
            } => (new_name) {
                RenameStmt::new(
                    RenameTarget::Conversion(conversion),
                    new_name,
                ).into()
            },
            {
                keyword(Set)
                    .and(keyword(Schema))
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
    use crate::parser::ast_node::RoleSpec::SessionUser;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

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
            "other_conversion".into(),
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
            "some_schema".into(),
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::Schema;
use crate::lexer::Keyword::Set;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::{Conversion, Owner};
use crate::parser::any_name;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterObjectSchemaTarget;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::col_id;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
use crate::parser::combinators::{keyword, match_first_with_state};
use crate::parser::role_parsers::role_spec;
