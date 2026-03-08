pub(in crate::combinators::stmt) fn tablespace(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        TABLESPACE ColId
    */

    let (_, name) = seq!(Tablespace, col_id)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_tablespace() {
        test_parser!(
            source = "tablespace foo",
            parser = tablespace,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Tablespace;
use pg_parser_core::scan;
