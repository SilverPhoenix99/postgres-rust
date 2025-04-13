pub(super) fn subscription() -> impl Combinator<Output = Str> {
    
    /*
        SUBSCRIPTION ColId
    */

    Subscription
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_subscription() {
        test_parser!(
            source = "subscription foo",
            parser = subscription(),
            expected = "foo".into()
        )
    }
}

use crate::lexer::Keyword::Subscription;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
