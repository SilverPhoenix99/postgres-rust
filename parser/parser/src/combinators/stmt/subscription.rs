pub(super) fn subscription() -> impl Combinator<Output = Str> {

    /*
        SUBSCRIPTION ColId
    */

    Subscription
        .and_right(parser(col_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_subscription() {
        test_parser!(
            source = "subscription foo",
            parser = subscription(),
            expected = "foo".into()
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::{parser, Combinator};
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
use pg_lexer::Keyword::Subscription;
