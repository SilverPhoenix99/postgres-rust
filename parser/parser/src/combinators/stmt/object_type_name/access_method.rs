/// Inlined: `drop_type_name`
pub(in crate::combinators::stmt) fn access_method(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        ACCESS METHOD ColId
    */

    let (.., name) = seq!(Access, Method, col_id).parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_access_method() {
        test_parser!(
            source = "access method foo",
            parser = access_method,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Access;
use pg_lexer::Keyword::Method;
use pg_parser_core::scan;
