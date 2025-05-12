pub(super) fn schema() -> impl Combinator<Output = Str> {

    /*
        SCHEMA ColId
    */

    Schema
        .and_right(col_id())
}

use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_basics::Str;
use postgres_parser_lexer::Keyword::Schema;
