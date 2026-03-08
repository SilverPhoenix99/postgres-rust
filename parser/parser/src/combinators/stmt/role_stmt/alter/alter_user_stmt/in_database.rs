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
    use crate::test_parser;

    #[test]
    fn test_in_database() {
        test_parser!(
            source = "in database db_name",
            parser = in_database,
            expected = "db_name"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Database;
use pg_lexer::Keyword::In;
use pg_parser_core::scan;
