/// Alias: `columnList`
pub(super) fn name_list(stream: &mut TokenStream) -> scan::Result<Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many_sep(Comma, col_id).parse(stream)
}

use crate::combinators::col_id;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::OperatorKind::Comma;
