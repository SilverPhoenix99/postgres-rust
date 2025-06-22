pub(super) fn schema() -> impl Combinator<Output = Str> {

    /*
        SCHEMA ColId
    */

    Schema
        .and_right(parser(col_id))
}

use crate::combinators::col_id;
use crate::combinators::foundation::{parser, Combinator};
use crate::combinators::foundation::CombinatorHelpers;
use pg_basics::Str;
use pg_lexer::Keyword::Schema;
