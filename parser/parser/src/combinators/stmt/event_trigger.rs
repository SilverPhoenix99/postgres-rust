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
    use pg_combinators::test_parser;

    #[test]
    fn test_event_trigger() {
        test_parser!(
            source = "event trigger foo",
            parser = event_trigger,
            expected = "foo"
        )
    }
}

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Event;
use pg_lexer::Keyword::Trigger;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::col_id;
