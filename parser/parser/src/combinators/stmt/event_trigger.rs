pub(super) fn event_trigger() -> impl Combinator<Output = Str> {

    /*
        EVENT TRIGGER ColId
    */

    and(Event, Trigger)
        .and_right(col_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_event_trigger() {
        test_parser!(
            source = "event trigger foo",
            parser = event_trigger(),
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::and;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::Keyword::Event;
use pg_lexer::Keyword::Trigger;
