pub(in crate::combinators::stmt) fn subscription(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        SUBSCRIPTION ColId
    */

    let (_, name) = seq!(Subscription, col_id)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

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
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Subscription;
use pg_parser_core::scan;
