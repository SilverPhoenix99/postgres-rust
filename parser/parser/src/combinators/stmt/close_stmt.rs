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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

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

use crate::combinators::col_id;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_ast::OneOrAll;
use pg_basics::Str;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Close;
