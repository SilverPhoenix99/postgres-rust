pub(super) fn role(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        ROLE name
    */

    let (_, name) = (Role, col_id)
        .parse(stream)?;

    Ok(name)
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
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Role;
