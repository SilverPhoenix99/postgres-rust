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

use crate::lexer::Keyword::Time;
use crate::lexer::Keyword::With;
use crate::lexer::Keyword::Without;
use crate::lexer::Keyword::Zone;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
