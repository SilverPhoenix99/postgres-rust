pub(in crate::combinators) fn var_name(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many!(stream => sep = Dot, col_id)
}

use crate::combinators::col_id;
use crate::combinators::foundation::many;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::OperatorKind::Dot;
