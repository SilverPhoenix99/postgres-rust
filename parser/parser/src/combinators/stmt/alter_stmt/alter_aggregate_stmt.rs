enum Change {
    Owner(RoleSpec),
    Name(Str),
    Schema(Str),
}

pub(super) fn alter_aggregate_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    /*
        ALTER AGGREGATE aggregate_with_argtypes (
            OWNER TO RoleSpec => AlterOwnerStmt
            RENAME TO ColId   => RenameStmt
            SET SCHEMA ColId  => AlterObjectSchemaStmt
        )
    */

    let (_, aggregate, change) = seq!(=>
        Aggregate.parse(stream),
        aggregate_with_argtypes(stream),
        choice!(stream =>
            seq!(stream => Owner, To, role_spec)
                .map(|(.., new_owner)| Change::Owner(new_owner)),
            seq!(stream => Rename, To, col_id)
                .map(|(.., new_name)| Change::Name(new_name)),
            seq!(stream => Set, Schema, col_id)
                .map(|(.., new_schema)| Change::Schema(new_schema))
        )
    )?;

    let stmt = match change {
        Change::Owner(new_owner) => {
            AlterOwnerStmt::new(
                AlterOwnerTarget::Aggregate(aggregate),
                new_owner
            ).into()
        },
        Change::Name(new_name) => {
            RenameStmt::new(
                RenameTarget::Aggregate(aggregate),
                new_name
            ).into()
        },
        Change::Schema(new_schema) => {
            AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Aggregate(aggregate),
                new_schema
            ).into()
        },
    };

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::AggregateWithArgs;
    use pg_ast::RoleSpec;

    #[test]
    fn test_alter_owner() {
        test_parser!(
            source = "aggregate aggregate_name(*) owner to current_user",
            parser = alter_aggregate_stmt,
            expected = AlterOwnerStmt::new(
                AlterOwnerTarget::Aggregate(
                    AggregateWithArgs::new(vec!["aggregate_name".into()], vec![], vec![])
                ),
                RoleSpec::CurrentUser
            )
        )
    }

    #[test]
    fn test_rename() {
        test_parser!(
            source = "aggregate aggregate_name(*) rename to different_name",
            parser = alter_aggregate_stmt,
            expected = RenameStmt::new(
                RenameTarget::Aggregate(
                    AggregateWithArgs::new(vec!["aggregate_name".into()], vec![], vec![])
                ),
                "different_name"
            )
        )
    }

    #[test]
    fn test_alter_schema() {
        test_parser!(
            source = "aggregate aggregate_name(*) set schema new_schema",
            parser = alter_aggregate_stmt,
            expected = AlterObjectSchemaStmt::new(
                AlterObjectSchemaTarget::Aggregate(
                    AggregateWithArgs::new(vec!["aggregate_name".into()], vec![], vec![])
                ),
                "new_schema"
            )
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::role::role_spec;
use crate::combinators::stmt::aggregate_with_argtypes;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::AlterObjectSchemaStmt;
use pg_ast::AlterObjectSchemaTarget;
use pg_ast::AlterOwnerStmt;
use pg_ast::AlterOwnerTarget;
use pg_ast::RawStmt;
use pg_ast::RenameStmt;
use pg_ast::RenameTarget;
use pg_ast::RoleSpec;
use pg_basics::Str;
use pg_lexer::Keyword::Aggregate;
use pg_lexer::Keyword::Owner;
use pg_lexer::Keyword::Rename;
use pg_lexer::Keyword::Schema;
use pg_lexer::Keyword::Set;
use pg_lexer::Keyword::To;
