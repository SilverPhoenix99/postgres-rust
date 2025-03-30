/// Alias: `DeallocateStmt`
pub(super) fn deallocate_stmt() -> impl Combinator<Output = OneOrAll<Str>> {

    /*
        DEALLOCATE (PREPARE)? ALL
        DEALLOCATE (PREPARE)? ColId
    */

    Deallocate
        .and(Prepare.optional())
        .and_right(or(
            All.map(|_| OneOrAll::All),
            col_id().map(OneOrAll::One)
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("deallocate all", OneOrAll::All)]
    #[test_case("deallocate prepare all", OneOrAll::All)]
    #[test_case("deallocate abort", OneOrAll::One("abort".into()))]
    #[test_case("deallocate prepare ident", OneOrAll::One("ident".into()))]
    fn test_deallocate(source: &str, expected: OneOrAll<Str>) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), deallocate_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::All;
use crate::lexer::Keyword::Deallocate;
use crate::lexer::Keyword::Prepare;
use crate::parser::ast_node::OneOrAll;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
