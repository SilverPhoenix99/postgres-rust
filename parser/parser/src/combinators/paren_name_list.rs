/// Post-condition: Vec is **Not** empty
///
/// Alias: `opt_column_list`
pub(super) fn paren_name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        '(' name_list ')'
    */

    between_paren(name_list())
}

use crate::combinators::between_paren;
use crate::combinators::foundation::Combinator;
use crate::combinators::name_list;
use pg_basics::Str;
