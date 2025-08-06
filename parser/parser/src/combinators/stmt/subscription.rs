pub(super) fn subscription(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        SUBSCRIPTION ColId
    */

    let (_, name) = seq!(Subscription, col_id)
        .parse(stream)?;

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

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_identifier_combinators::col_id;
use pg_lexer::Keyword::Subscription;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
