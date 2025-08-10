pub fn role(ctx: &mut ParserContext) -> scan::Result<Str> {

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
    use pg_combinators::test_parser;

    #[test]
    fn test_role() {
        test_parser!(
            source = "role foo",
            parser = role,
            expected = "foo"
        )
    }
}

use crate::col_id;
use pg_basics::Str;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Role;
use pg_parser_core::scan;
