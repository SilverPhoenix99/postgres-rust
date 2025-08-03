pg_basics::reexport! {
    create_access_method_stmt,
    create_cast_stmt,
    create_conversion_stmt,
    create_database_stmt,
    create_role_option,
    create_role_stmt,
    create_user_stmt,
}

pub(super) use create_database_stmt::createdb_opt_value;

pub(super) fn create_stmt(stream: &mut TokenStream) -> scan::Result<RawStmt> {

    let (_, stmt) = seq!(
        Create,
        alt!(
            create_access_method_stmt.map(From::from),
            create_cast_stmt.map(From::from),
            create_conversion_stmt.map(From::from),
            create_database_stmt.map(From::from),
            create_role_stmt.map(From::from),
            create_user_stmt,
        )
    ).parse(stream)?;

    Ok(stmt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("create access method foo type table handler bar")]
    #[test_case("create cast (int as text) with inout")]
    #[test_case("create conversion conv_name for 'for-encoding' to 'to-encoding' from func_name")]
    #[test_case("create database new_db oid = 1")]
    #[test_case("create role new_role with superuser")]
    #[test_case("create user new_user with password 'password'")]
    fn test_create_stmt(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = create_stmt(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::RawStmt;
use pg_lexer::Keyword::Create;
