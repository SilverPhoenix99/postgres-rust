pub(in crate::combinators::stmt) fn index(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

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
    use crate::test_parser;

    #[test]
    fn test_index() {
        test_parser!(
            source = "index foo",
            parser = index,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Index;
use pg_parser_core::scan;
