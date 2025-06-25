/// Alias: `columnList`
pub(super) fn name_list(stream: &mut TokenStream) -> Result<Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many!(stream => sep = Comma, col_id)
}

use crate::combinators::col_id;
use crate::combinators::foundation::many;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::OperatorKind::Comma;
