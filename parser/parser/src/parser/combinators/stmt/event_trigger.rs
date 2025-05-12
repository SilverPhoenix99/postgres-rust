pub(super) fn event_trigger() -> impl Combinator<Output = Str> {
    
    /*
        EVENT TRIGGER ColId
    */

    and(Event, Trigger)
        .and_right(col_id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_event_trigger() {
        test_parser!(
            source = "event trigger foo",
            parser = event_trigger(),
            expected = "foo".into()
        )
    }
}

use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::Event;
use postgres_parser_lexer::Keyword::Trigger;
