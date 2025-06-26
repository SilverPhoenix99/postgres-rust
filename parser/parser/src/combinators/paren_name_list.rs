/// Alias: `opt_column_list`
pub(super) fn paren_name_list(stream: &mut TokenStream) -> scan::Result<Vec<Str>> {

    /*
        '(' name_list ')'
    */

    between!(paren : stream => name_list(stream))
}

use crate::combinators::foundation::between;
use crate::combinators::name_list;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
