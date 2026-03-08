pub(in crate::combinators::stmt) fn role(ctx: &mut ParserContext) -> scan::Result<Str> {

    /*
        ROLE name
    */

    let (_, name) = seq!(Role, col_id)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_role() {
        test_parser!(
            source = "role foo",
            parser = role,
            expected = "foo"
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::Str;
use pg_lexer::Keyword::Role;
use pg_parser_core::scan;
