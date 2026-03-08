pub(in crate::combinators::stmt) fn collation(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        COLLATION any_name
    */

    let (_, name) = seq!(Collation, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_collation() {
        test_parser!(
            source = "collation foo",
            parser = collation,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Collation;
use pg_parser_core::scan;
