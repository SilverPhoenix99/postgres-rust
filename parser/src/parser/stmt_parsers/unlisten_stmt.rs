/// Alias: `UnlistenStmt`
pub(in crate::parser) fn unlisten_stmt() -> impl Combinator<Output = OneOrAll> {

    /*
        UNLISTEN '*'
        UNLISTEN ColId
    */
    
    keyword(Unlisten)
        .and_right(match_first!{
            operator(Mul).map(|_| OneOrAll::All),
            col_id().map(OneOrAll::Name)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use test_case::test_case;

    #[test_case("unlisten *", OneOrAll::All)]
    #[test_case("unlisten test_name", OneOrAll::Name("test_name".into()))]
    fn test_unlisten(source: &str, expected: OneOrAll) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), unlisten_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Unlisten;
use crate::lexer::OperatorKind::Mul;
use crate::parser::ast_node::OneOrAll;
use crate::parser::col_id;
use crate::parser::combinators::{keyword, match_first, Combinator};
use crate::parser::combinators::{operator, CombinatorHelpers};
