/// Alias: `LoadStmt`
pub(super) fn load_stmt() -> impl Combinator<Output = Box<str>> {

    /*
        LOAD SCONST
    */

    Load
        .and_right(string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;

    #[test]
    fn test_load_stmt() {
        let mut stream = TokenStream::new("load 'test string'", DEFAULT_CONFIG);
        assert_eq!(Ok("test string".into()), load_stmt().parse(&mut stream));
    }
}

use crate::combinators::foundation::string;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Load;
