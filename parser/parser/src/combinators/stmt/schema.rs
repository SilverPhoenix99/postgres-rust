pub(super) fn schema(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        SCHEMA ColId
    */

    let (_, name) = seq!(Schema, col_id)
        .parse(stream)?;

    Ok(name)
}

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Schema;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::col_id;
