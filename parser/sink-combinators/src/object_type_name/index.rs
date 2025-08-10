pub fn index(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        INDEX any_name
    */

    let (_, name) = seq!(Index, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_index() {
        test_parser!(
            source = "index foo",
            parser = index,
            expected = vec!["foo".into()]
        )
    }
}

use crate::any_name;
use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Index;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
