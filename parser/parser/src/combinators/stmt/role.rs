pub(super) fn role(stream: &mut TokenStream) -> Result<Str> {

    /*
        ROLE name
    */

    seq!(stream => Role, col_id)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_role() {
        test_parser!(
            source = "role foo",
            parser = role,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Role;
