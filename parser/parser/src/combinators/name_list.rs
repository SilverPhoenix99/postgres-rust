/// Alias: `columnList`
pub(super) fn name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    parser(|stream|
        many!(
            sep = Comma.parse(stream),
            col_id(stream)
        )
    )
}

use crate::combinators::foundation::many;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::combinators::v2::col_id;
use pg_basics::Str;
use pg_lexer::OperatorKind::Comma;
