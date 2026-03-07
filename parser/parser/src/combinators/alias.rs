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
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::As;
use pg_parser_core::scan;
