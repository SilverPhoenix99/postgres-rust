/// Alias: `columnList`
pub(super) fn name_list(stream: &mut TokenStream) -> scan::Result<Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many!(sep = Comma, col_id).parse(stream)
}

use crate::combinators::col_id;
use pg_basics::Str;
use pg_combinators::many;
use pg_combinators::Combinator;
use pg_lexer::OperatorKind::Comma;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
