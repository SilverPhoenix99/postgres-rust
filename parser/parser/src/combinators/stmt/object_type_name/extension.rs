pub(in crate::combinators::stmt) fn extension(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        EXTENSION ColId
    */

    let (_, name) = seq!(Extension, col_id).parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_extension() {
        test_parser!(
            source = "extension foo",
            parser = extension,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Extension;
use pg_parser_core::scan;
