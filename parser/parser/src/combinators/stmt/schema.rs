pub(super) fn schema(stream: &mut TokenStream) -> Result<Str> {

    /*
        SCHEMA ColId
    */

    let (_, name) = seq!(stream => Schema, col_id)?;

    Ok(name)
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::Schema;
