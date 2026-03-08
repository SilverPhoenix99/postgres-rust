pg_basics::reexport! {
    create_access_method_stmt,
    create_cast_stmt,
}

pub(super) fn create_stmt(ctx: &mut ParserContext) -> scan::Result<RawStmt> {

    let (_, stmt) = seq!(
        Create,
        alt!(
            create_access_method_stmt.map(From::from),
            create_cast_stmt.map(From::from),
            create_conversion_stmt.map(From::from),
            create_database_stmt.map(From::from),
            create_role_stmt.map(From::from),
            create_user_stmt.map(From::from),
        )
    ).parse(ctx)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix(
        [
            "create access method foo type table handler bar",
            "create cast (int as text) with inout",
            "create conversion conv_name for 'for-encoding' to 'to-encoding' from func_name",
            "create database new_db oid = 1",
            "create role new_role with superuser",
            "create user new_user with password 'password'",
        ]
        => matches Ok(_)
    )]
    fn test_create_stmt(source: &str) -> scan::Result<RawStmt> {
        test_parser!(source, create_stmt)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::stmt::conversion_stmt::create_conversion_stmt;
use crate::combinators::stmt::database_stmt::create_database_stmt;
use crate::combinators::stmt::role_stmt::create_role_stmt;
use crate::combinators::stmt::role_stmt::create_user_stmt;
use crate::seq;
use crate::ParserContext;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Create;
use pg_parser_core::scan;
