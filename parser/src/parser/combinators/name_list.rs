/// Post-condition: Vec is **Not** empty
///
/// Alias: `columnList`
pub(super) fn name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many_sep(Comma, col_id::col_id())
}

use crate::lexer::OperatorKind::Comma;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::many_sep;
use crate::parser::combinators::foundation::Combinator;
use postgres_basics::Str;
