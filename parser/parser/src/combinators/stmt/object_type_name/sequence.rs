pub(in crate::combinators::stmt) fn sequence(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        SEQUENCE any_name
    */

    let (_, name) = seq!(Sequence, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_sequence() {
        test_parser!(
            source = "sequence foo",
            parser = sequence,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Sequence;
use pg_parser_core::scan;
