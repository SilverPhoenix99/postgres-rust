pub(super) fn opt_timezone() -> impl Combinator<Output = bool> {

    /*
        ( (WITH | WITHOUT) TIME ZONE )?
    */

    match_first!(
        With.map(|_| true),
        Without.map(|_| false)
    )
        .and_left(sequence!(Time, Zone).skip())
        .optional()
        .map(|tz| tz.unwrap_or(false))
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::Time;
use postgres_parser_lexer::Keyword::With;
use postgres_parser_lexer::Keyword::Without;
use postgres_parser_lexer::Keyword::Zone;
