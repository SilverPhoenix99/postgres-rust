/// Alias: `DeallocateStmt`
pub(in crate::parser) fn deallocate_stmt() -> impl Combinator<Output = OneOrAll> {

    /*
        DEALLOCATE (PREPARE)? ALL
        DEALLOCATE (PREPARE)? ColId
    */

    keyword(Deallocate)
        .and(keyword(Prepare).optional())
        .and_right(or(
            keyword(All).map(|_| OneOrAll::All),
            col_id().map(OneOrAll::Name)
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
    #[test_case("deallocate abort", OneOrAll::Name("abort".into()))]
    #[test_case("deallocate prepare ident", OneOrAll::Name("ident".into()))]
    fn test_deallocate(source: &str, expected: OneOrAll) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), deallocate_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Prepare;
use crate::lexer::Keyword::{All, Deallocate};
use crate::parser::ast_node::OneOrAll;
use crate::parser::col_id;
use crate::parser::combinators::{keyword, or, Combinator, CombinatorHelpers};
