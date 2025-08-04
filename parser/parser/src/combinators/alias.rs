/// Aliases:
/// * `json_table_path_name_opt`
/// * `opt_alias_clause_for_join_using`
pub(super) fn alias(stream: &mut TokenStream) -> scan::Result<Str> {

    /*
        AS col_id
    */

    let (_, alias) = seq!(As, col_id)
        .parse(stream)?;

    Ok(alias)
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::Str;
use pg_lexer::Keyword::As;
