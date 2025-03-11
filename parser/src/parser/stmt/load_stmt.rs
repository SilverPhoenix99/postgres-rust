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
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;

    #[test]
    fn test_load_stmt() {
        let mut stream = TokenStream::new("load 'test string'", DEFAULT_CONFIG);
        assert_eq!(Ok("test string".into()), load_stmt().parse(&mut stream));
    }
}

use crate::lexer::Keyword::Load;
use crate::parser::combinators::string;
use crate::parser::combinators::Combinator;
use crate::parser::combinators::CombinatorHelpers;
