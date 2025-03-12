mod create_database_stmt;

pub(super) use create_database_stmt::createdb_opt_value;

pub(super) fn create_stmt() -> impl Combinator<Output = RawStmt> {

    Create.and_right(match_first! {
        create_database_stmt().map(From::from),
        parser(|_| todo!())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("create database new_db oid = 1")]
    fn test_create_stmt(source: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = create_stmt().parse(&mut stream);

        // This only quickly tests that statement types aren't missing.
        // More in-depth testing is within each statement's module.
        assert_matches!(actual, Ok(_),
            r"expected Ok(Some(_)) for {source:?} but actually got {actual:?}"
        )
    }
}

use self::create_database_stmt::create_database_stmt;
use crate::lexer::Keyword::Create;
use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::parser;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
