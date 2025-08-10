enum Change {
    Owner(RoleSpec),
    Name(Str),
    Schema(Str),
}

pub(super) fn alter_conversion_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    /*
        ALTER CONVERSION any_name OWNER TO RoleSpec
        ALTER CONVERSION any_name RENAME TO ColId
        ALTER CONVERSION any_name SET SCHEMA ColId
    */

    let (_, name, change) = seq!(Conversion, any_name, changes).parse(ctx)?;

    let stmt = match change {
        Change::Owner(new_owner) => {
            AlterOwnerStmt::new(
                AlterOwnerTarget::Conversion(name),
                new_owner
            ).into()
        },
        Change::Name(new_name) => {
            RenameStmt::new(
                RenameTarget::Conversion(name),
                new_name
            ).into()
        },
        Change::Schema(new_schema) => {
            AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Conversion(name),
                new_schema
            ).into()
        },
    };

    Ok(stmt)
}

fn changes(ctx: &mut ParserContext) -> scan::Result<Change> {
    alt!(
        seq!(Owner, To, role_spec)
            .map(|(.., new_owner)| Change::Owner(new_owner)),
        seq!(Rename, To, col_id)
            .map(|(.., new_name)| Change::Name(new_name)),
        seq!(Set, Schema, col_id)
            .map(|(.., new_schema)| Change::Schema(new_schema))
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_sink_ast::RoleSpec::SessionUser;

    #[test]
    fn test_alter_conversion_owner() {
        test_parser!(
            source = "conversion conversion_name owner to session_user",
            parser = alter_conversion_stmt,
            expected = AlterOwnerStmt::new(
                AlterOwnerTarget::Conversion(vec!["conversion_name".into()]),
                SessionUser
            )
        )
    }

    #[test]
    fn test_alter_conversion_rename() {
        test_parser!(
            source = "conversion conversion_name rename to other_conversion",
            parser = alter_conversion_stmt,
            expected = RenameStmt::new(
                RenameTarget::Conversion(vec!["conversion_name".into()]),
                "other_conversion"
            )
        )
    }

    #[test]
    fn test_alter_conversion_schema() {
        test_parser!(
            source = "conversion conversion_name set schema some_schema",
            parser = alter_conversion_stmt,
            expected = AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Conversion(vec!["conversion_name".into()]),
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
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_basics::Str;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Conversion;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_ast::RoleSpec;
use pg_sink_combinators::any_name;
use pg_sink_combinators::col_id;
use pg_sink_combinators::role_spec;
