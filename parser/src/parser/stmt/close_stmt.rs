/// Alias: `ClosePortalStmt`
pub(in crate::parser) fn close_stmt() -> impl Combinator<Output = OneOrAll> {

    /*
        CLOSE ALL
        CLOSE ColId
    */

    keyword(Close)
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

    #[test]
    fn test_close_all() {
        let mut stream = TokenStream::new("close all", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), close_stmt().parse(&mut stream));
    }

    #[test]
    fn test_close_named() {
        let mut stream = TokenStream::new("close abort", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("abort".into())), close_stmt().parse(&mut stream));

        let mut stream = TokenStream::new("close ident", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::Name("ident".into())), close_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::{All, Close};
use crate::parser::ast_node::OneOrAll;
use crate::parser::col_id;
use crate::parser::combinators::{keyword, or, Combinator, CombinatorHelpers};
