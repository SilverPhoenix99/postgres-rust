/// Alias: `AlterCollationStmt`
pub(super) fn alter_collation_stmt() -> impl Combinator<Output = RawStmt> {

    /*
        ALTER COLLATION any_name REFRESH VERSION_P
        ALTER COLLATION any_name OWNER TO RoleSpec
        ALTER COLLATION any_name RENAME TO ColId
        ALTER COLLATION any_name SET SCHEMA ColId
    */

    Collation
        .and_right(any_name())
        .chain(match_first_with_state!{|name, stream| {
            {
                Refresh.and(Version)
            } => (_) {
                RefreshCollationVersionStmt(name)
            },
            {
                Owner.and(To)
                    .and_right(role_spec())
            } => (new_owner) {
                AlterOwnerStmt::new(
                    AlterOwnerTarget::Collation(name),
                    new_owner
                ).into()
            },
            {
                Rename.and(To)
                    .and_right(col_id())
            } => (new_name) {
                RenameStmt::new(
                    RenameTarget::Collation(name),
                    new_name
                ).into()
            },
            {
                Set.and(Schema)
                    .and_right(col_id())
            } => (new_schema) {
                AlterObjectSchemaStmt::new(
                    AlterObjectSchemaTarget::Collation(name),
                    new_schema
                ).into()
            }
        }})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::RawStmt::RefreshCollationVersionStmt;
    use crate::parser::ast_node::RoleSpec::CurrentUser;
    use crate::parser::ast_node::{AlterObjectSchemaTarget, AlterOwnerTarget, RenameTarget};
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_collation_owner() {
        let source = "collation collation_name owner to current_user";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = alter_collation_stmt().parse(&mut stream);

        let expected = AlterOwnerStmt::new(
            AlterOwnerTarget::Collation(vec!["collation_name".into()]),
            CurrentUser
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_collation_refresh_version() {
        let source = "collation collation_name refresh version";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = alter_collation_stmt().parse(&mut stream);
        let expected = RefreshCollationVersionStmt(vec!["collation_name".into()]);
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_collation_rename() {
        let source = "collation collation_name rename to something_else";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = alter_collation_stmt().parse(&mut stream);

        let expected = RenameStmt::new(
            RenameTarget::Collation(vec!["collation_name".into()]),
            "something_else"
        );

        assert_eq!(Ok(expected.into()), actual);
    }

    #[test]
    fn test_collation_set_schema() {
        let source = "collation collation_name set schema some_schema";
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);

        let actual = alter_collation_stmt().parse(&mut stream);

        let expected = AlterObjectSchemaStmt::new(
            AlterObjectSchemaTarget::Collation(vec!["collation_name".into()]),
            "some_schema"
        );

        assert_eq!(Ok(expected.into()), actual);
    }
}

use crate::lexer::Keyword::Collation;
use crate::lexer::Keyword::Owner;
use crate::lexer::Keyword::Refresh;
use crate::lexer::Keyword::Rename;
use crate::lexer::Keyword::Schema;
use crate::lexer::Keyword::Set;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::Version;
use crate::parser::ast_node::AlterObjectSchemaStmt;
use crate::parser::ast_node::AlterObjectSchemaTarget;
use crate::parser::ast_node::AlterOwnerStmt;
use crate::parser::ast_node::AlterOwnerTarget;
use crate::parser::ast_node::RawStmt;
use crate::parser::ast_node::RawStmt::RefreshCollationVersionStmt;
use crate::parser::ast_node::RenameStmt;
use crate::parser::ast_node::RenameTarget;
use crate::parser::combinators::any_name;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::match_first_with_state;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::role_spec;
