pub(in crate::combinators::stmt) fn database(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        DATABASE ColId
    */

    let (_, name) = seq!(Database, col_id).parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_database() {
        test_parser!(
            source = "database foo",
            parser = database,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Database;
use pg_parser_core::scan;
