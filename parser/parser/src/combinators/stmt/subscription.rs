pub(super) fn subscription(stream: &mut TokenStream) -> Result<Str> {

    /*
        SUBSCRIPTION ColId
    */

    let (_, name) = seq!(stream => Subscription, col_id)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_subscription() {
        test_parser!(
            source = "subscription foo",
            parser = subscription,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Subscription;
