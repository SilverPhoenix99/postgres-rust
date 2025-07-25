pub(in crate::combinators) fn var_name(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many_sep(Dot, col_id).parse(stream)
}

use crate::combinators::col_id;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::OperatorKind::Dot;
