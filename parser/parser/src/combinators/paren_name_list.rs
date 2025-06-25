/// Alias: `opt_column_list`
pub(super) fn paren_name_list(stream: &mut TokenStream) -> Result<Vec<Str>> {

    /*
        '(' name_list ')'
    */

    between!(paren : stream => name_list.parse(stream))
}

use crate::combinators::foundation::between;
use crate::combinators::foundation::Combinator;
use crate::combinators::name_list;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::Str;
