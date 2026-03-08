pub(in crate::combinators::stmt) fn event_trigger(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        EVENT TRIGGER ColId
    */

    let (.., name) = seq!(Event, Trigger, col_id)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

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
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Event;
use pg_lexer::Keyword::Trigger;
use pg_parser_core::scan;
