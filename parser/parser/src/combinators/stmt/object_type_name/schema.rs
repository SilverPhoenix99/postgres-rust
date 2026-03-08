pub(in crate::combinators::stmt) fn schema(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        SCHEMA ColId
    */

    let (_, name) = seq!(Schema, col_id)
        .parse(ctx)?;

    Ok(name)
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Schema;
use pg_parser_core::scan;
