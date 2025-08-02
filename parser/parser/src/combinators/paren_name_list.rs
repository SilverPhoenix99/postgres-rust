/// Aliases:
/// * `opt_column_list`
/// * `opt_name_list`
pub(super) fn paren_name_list(stream: &mut TokenStream) -> scan::Result<Vec<Str>> {

    /*
        '(' name_list ')'
    */

    paren(name_list).parse(stream)
}

use crate::combinators::foundation::paren;
use crate::combinators::foundation::Combinator;
use crate::combinators::name_list;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
