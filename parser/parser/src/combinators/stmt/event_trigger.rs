pub(super) fn event_trigger(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        EVENT TRIGGER ColId
    */

    let (.., name) = seq!(Event, Trigger, col_id)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_event_trigger() {
        test_parser!(
            source = "event trigger foo",
            parser = event_trigger,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Event;
use pg_lexer::Keyword::Trigger;
