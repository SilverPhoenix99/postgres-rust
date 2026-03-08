/// Aliases:
/// * `json_table_path_name_opt`
/// * `opt_alias_clause_for_join_using`
pub(super) fn alias(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        AS col_id
    */

    let (_, alias) = seq!(As, col_id)
        .parse(ctx)?;

    Ok(alias)
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::As;
use pg_parser_core::scan;
