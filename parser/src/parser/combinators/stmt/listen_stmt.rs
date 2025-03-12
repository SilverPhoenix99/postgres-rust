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
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_listen_stmt() {
        let mut stream = TokenStream::new("listen abort", DEFAULT_CONFIG);
        assert_eq!(Ok("abort".into()), listen_stmt().parse(&mut stream));

        let mut stream = TokenStream::new("listen ident", DEFAULT_CONFIG);
        assert_eq!(Ok("ident".into()), listen_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Listen;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
