/// Alias: `ListenStmt`
pub(super) fn listen_stmt(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        LISTEN ColId
    */

    let (_, channel) = seq!(Listen, col_id)
        .parse(stream)?;

    Ok(channel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("listen abort", "abort".into())]
    #[test_case("listen ident", "ident".into())]
    fn test_listen_stmt(source: &str, expected: Str) {
        test_parser!(source, listen_stmt, expected)
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Listen;
