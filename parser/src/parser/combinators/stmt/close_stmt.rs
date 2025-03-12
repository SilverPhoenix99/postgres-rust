/// Alias: `ClosePortalStmt`
pub(super) fn close_stmt() -> impl Combinator<Output = OneOrAll<Str>> {

    /*
        CLOSE ALL
        CLOSE ColId
    */

    Close
        .and_right(or(
            All.map(|_| OneOrAll::All),
            col_id().map(OneOrAll::One)
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_close_all() {
        let mut stream = TokenStream::new("close all", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::All), close_stmt().parse(&mut stream));
    }

    #[test]
    fn test_close_named() {
        let mut stream = TokenStream::new("close abort", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::One("abort".into())), close_stmt().parse(&mut stream));

        let mut stream = TokenStream::new("close ident", DEFAULT_CONFIG);
        assert_eq!(Ok(OneOrAll::One("ident".into())), close_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::All;
use crate::lexer::Keyword::Close;
use crate::parser::ast_node::OneOrAll;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
