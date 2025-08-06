enum Change {
    RefreshVersion,
    Owner(RoleSpec),
    Name(Str),
    Schema(Str),
}

/// Alias: `AlterCollationStmt`
pub(super) fn alter_collation_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        ALTER COLLATION any_name REFRESH VERSION_P
        ALTER COLLATION any_name OWNER TO RoleSpec
        ALTER COLLATION any_name RENAME TO ColId
        ALTER COLLATION any_name SET SCHEMA ColId
    */

    let (_, name, change) = seq!(Collation, any_name, changes).parse(stream)?;

    let stmt = match change {
        Change::RefreshVersion => {
            RefreshCollationVersionStmt(name)
        }
        Change::Owner(new_owner) => {
            AlterOwnerStmt::new(
                AlterOwnerTarget::Collation(name),
                new_owner
            ).into()
        }
        Change::Name(new_name) => {
            RenameStmt::new(
                RenameTarget::Collation(name),
                new_name
            ).into()
        }
        Change::Schema(new_schema) => {
            AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Collation(name),
                new_schema
            ).into()
        }
    };

    Ok(stmt)
}

fn changes(stream: &mut TokenStream) -> scan::Result<Change> {
    alt!(
        seq!(Refresh, Version)
            .map(|_| Change::RefreshVersion),
        seq!(Owner, To, role_spec)
            .map(|(.., role)| Change::Owner(role)),
        seq!(Rename, To, col_id)
            .map(|(.., name)| Change::Name(name)),
        seq!(Set, Schema, col_id)
            .map(|(.., schema)| Change::Schema(schema))
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::RoleSpec::CurrentUser;

    #[test]
    fn test_collation_owner() {
        test_parser!(
            source = "collation collation_name owner to current_user",
            parser = alter_collation_stmt,
            expected = AlterOwnerStmt::new(
                AlterOwnerTarget::Collation(vec!["collation_name".into()]),
                CurrentUser
            )
        )
    }

    #[test]
    fn test_collation_refresh_version() {
        test_parser!(
            source = "collation collation_name refresh version",
            parser = alter_collation_stmt,
            expected = RefreshCollationVersionStmt(vec!["collation_name".into()])
        )
    }

    #[test]
    fn test_collation_rename() {
        test_parser!(
            source = "collation collation_name rename to something_else",
            parser = alter_collation_stmt,
            expected = RenameStmt::new(
                RenameTarget::Collation(vec!["collation_name".into()]),
                "something_else"
            )
        )
    }

    #[test]
    fn test_collation_set_schema() {
        test_parser!(
            source = "collation collation_name set schema some_schema",
            parser = alter_collation_stmt,
            expected = AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Collation(vec!["collation_name".into()]),
                "some_schema"
            )
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::role_spec;
use pg_ast::AlterObjectSchemaStmt;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RawStmt::RefreshCollationVersionStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_ast::RoleSpec;
use pg_basics::Str;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Refresh;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::Version;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
