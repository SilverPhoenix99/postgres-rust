pg_basics::reexport! {
    alter_aggregate_stmt,
    alter_default_privileges_stmt,
    alter_extension_stmt,
    alter_function_stmt,
}

pub(super) fn alter_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    let (_, stmt) = seq!(
        Alter,
        alt!(
            alter_aggregate_stmt,
            alter_collation_stmt,
            alter_conversion_stmt,
            alter_database_stmt.map(From::from),
            alter_default_privileges_stmt.map(From::from),
            alter_event_trigger_stmt,
            alter_extension_stmt,
            alter_function_stmt,
            alter_group_stmt.map(From::from),
            alter_language_stmt,
            alter_large_object_stmt,
            alter_system_stmt.map(From::from),
            alter_user_stmt.map(From::from),
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_matrix;

    // This only quickly tests that statement types aren't missing.
    // More in-depth testing is within each statement's module.
    #[test_matrix(
        [
            "alter aggregate aggregate_name(*) owner to current_user",
            "alter collation some_name refresh version",
            "alter conversion some_conversion rename to new_conversion",
            "alter database the_db refresh collation version",
            "alter default privileges in schema some_schema grant all on tables to public",
            "alter event trigger some_trigger owner to current_user",
            "alter extension foo set schema some_schema",
            "alter function some_function() owner to current_user",
            "alter group some_group rename to new_group_name",
            "alter language lang owner to session_user",
            "alter large object -127 owner to public",
            "alter system reset all",
            "alter user public",
        ]
        => matches Ok(_)
    )]
    fn test_alter_stmt(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, alter_stmt)
    }
}

use pg_ast::RawStmt;
use pg_collation_stmt::alter::alter_collation_stmt;
use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_conversion_stmt::alter::alter_conversion_stmt;
use pg_database_stmt::alter::alter_database_stmt;
use pg_event_trigger_stmt::alter::alter_event_trigger_stmt;
use pg_language_stmt::alter::alter_language_stmt;
use pg_large_object_stmt::alter::alter_large_object_stmt;
use pg_lexer::Keyword::Alter;
use pg_parser_core::scan;
use pg_role_stmt::alter::alter_group_stmt;
use pg_role_stmt::alter::alter_user_stmt;
use pg_system_stmt::alter::alter_system_stmt;
