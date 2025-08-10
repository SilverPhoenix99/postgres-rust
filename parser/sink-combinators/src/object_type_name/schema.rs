pub fn schema(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        SCHEMA ColId
    */

    let (_, name) = seq!(Schema, col_id)
        .parse(ctx)?;

    Ok(name)
}

use crate::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Schema;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
