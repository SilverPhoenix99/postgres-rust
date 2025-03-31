/// Post-condition: Vec is **Not** empty
///
/// Alias: `opt_column_list`
pub(super) fn paren_name_list() -> impl Combinator<Output = Vec<Str>> {

    /*
        '(' name_list ')'
    */

    between_paren(name_list())
}

use crate::parser::combinators::between_paren;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::name_list;
use postgres_basics::Str;
