/// Alias: `ListenStmt`
pub(super) fn listen_stmt() -> impl Combinator<Output = Str> {

    /*
        LISTEN ColId
    */

    Listen
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_listen_stmt() {
        let mut stream = TokenStream::new("listen abort", DEFAULT_CONFIG);
        assert_eq!(Ok("abort".into()), listen_stmt().parse(&mut stream));

        let mut stream = TokenStream::new("listen ident", DEFAULT_CONFIG);
        assert_eq!(Ok("ident".into()), listen_stmt().parse(&mut stream));
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::Listen;
