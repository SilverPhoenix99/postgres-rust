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
use pg_basics::Str;
use pg_combinators::Combinator;
use pg_lexer::Keyword::As;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
