enum Change {
    RefreshVersion,
    Owner(RoleSpec),
    Name(Str),
    Schema(Str),
}

/// Alias: `AlterCollationStmt`
pub fn alter_collation_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        ALTER COLLATION any_name REFRESH VERSION_P
        ALTER COLLATION any_name OWNER TO RoleSpec
        ALTER COLLATION any_name RENAME TO ColId
        ALTER COLLATION any_name SET SCHEMA ColId
    */

    let (_, name, change) = seq!(Collation, any_name, change).parse(ctx)?;

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

fn change(ctx: &mut ParserContext) -> scan::Result<Change> {
    alt!(
        seq!(Refresh, Version)
            .map(|_| Change::RefreshVersion),
        seq!(Owner, To, role_spec)
            .map(|(.., role)| Change::Owner(role)),
        seq!(Rename, To, col_id)
            .map(|(.., name)| Change::Name(name)),
        seq!(Set, Schema, col_id)
            .map(|(.., schema)| Change::Schema(schema))
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::RoleSpec::CurrentUser;

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

use pg_ast::AlterObjectSchemaStmt;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RawStmt::RefreshCollationVersionStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Collation;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Refresh;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::Version;
use pg_parser_core::scan;
use pg_sink_ast::RoleSpec;
use pg_sink_combinators::any_name;
use pg_sink_combinators::col_id;
use pg_sink_combinators::role_spec;
