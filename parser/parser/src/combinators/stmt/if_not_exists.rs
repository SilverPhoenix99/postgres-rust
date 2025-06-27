pub(super) fn if_not_exists(stream: &mut TokenStream) -> scan::Result<bool> {

    let opt = seq!(stream => If, Not, Exists)
        .optional()?;

    Ok(opt.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("if not exists", true)]
    #[test_case("", false)]
    #[test_case("something else", false)]
    fn test_(source: &str, expected: bool) {
        test_parser!(source, if_not_exists, expected)
    }
}

use crate::combinators::foundation::seq;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Exists;
use pg_lexer::Keyword::If;
use pg_lexer::Keyword::Not;
