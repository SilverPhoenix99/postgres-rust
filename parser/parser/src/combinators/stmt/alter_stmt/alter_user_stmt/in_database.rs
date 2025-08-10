/// Alias: `opt_in_database`
pub(super) fn in_database(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        IN DATABASE col_id
    */

    let (.., dbname) = seq!(In, Database, col_id).parse(ctx)?;
    Ok(dbname)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_in_database() {
        test_parser!(
            source = "in database db_name",
            parser = in_database,
            expected = "db_name"
        )
    }
}

use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::In;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_combinators::col_id;
