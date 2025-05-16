/// Post-condition: Vec is **Not** empty
///
/// Alias: `columnList`
pub(super) fn name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        col_id ( ',' col_id )*
    */

    many_sep(Comma, col_id())
}

use crate::combinators::col_id;
use crate::combinators::foundation::many_sep;
use crate::combinators::foundation::Combinator;
use pg_basics::Str;
use pg_lexer::OperatorKind::Comma;
