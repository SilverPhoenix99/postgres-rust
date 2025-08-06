pub fn var_name(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        col_id ( '.' col_id )*
    */

    many!(sep = Dot, col_id).parse(stream)
}

use crate::col_id;
use pg_basics::QualifiedName;
use pg_combinators::many;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Dot;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
