pub(super) fn schema(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        SCHEMA ColId
    */

    let (_, name) = seq!(Schema, col_id)
        .parse(stream)?;

    Ok(name)
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::Keyword::Schema;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
