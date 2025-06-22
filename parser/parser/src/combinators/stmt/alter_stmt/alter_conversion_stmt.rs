enum Change {
    Owner(RoleSpec),
    Name(Str),
    Schema(Str),
}

pub(super) fn alter_conversion_stmt(stream: &mut TokenStream) -> Result<RawStmt> {

    /*
        ALTER CONVERSION any_name OWNER TO RoleSpec
        ALTER CONVERSION any_name RENAME TO ColId
        ALTER CONVERSION any_name SET SCHEMA ColId
    */

    Conversion
        .and_right(seq!(
            any_name,
            choice!(
                seq!(Owner, To, role_spec)
                    .map(|(.., new_owner)| Change::Owner(new_owner)),
                seq!(Rename, To, col_id)
                    .map(|(.., new_name)| Change::Name(new_name)),
                seq!(Set, Schema, col_id)
                    .map(|(.., new_schema)| Change::Schema(new_schema))
            )
        ))
        .map(|(name, change)| match change {
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
        })
        .parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::RoleSpec::SessionUser;

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

use crate::combinators::any_name;
use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::role_spec;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_ast::{AlterObjectSchemaStmt, RoleSpec};
use pg_basics::Str;
use pg_lexer::Keyword::Conversion;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
