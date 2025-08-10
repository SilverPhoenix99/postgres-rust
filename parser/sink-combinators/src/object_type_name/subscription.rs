pub fn subscription(ctx: &mut ParserContext) -> scan::Result<Str> {

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
    use pg_combinators::test_parser;

    #[test]
    fn test_subscription() {
        test_parser!(
            source = "subscription foo",
            parser = subscription,
            expected = "foo"
        )
    }
}

use crate::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Subscription;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
