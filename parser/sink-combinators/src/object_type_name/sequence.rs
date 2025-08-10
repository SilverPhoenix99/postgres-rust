pub fn sequence(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

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
    use pg_combinators::test_parser;

    #[test]
    fn test_sequence() {
        test_parser!(
            source = "sequence foo",
            parser = sequence,
            expected = vec!["foo".into()]
        )
    }
}

use crate::any_name;
use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Sequence;
use pg_parser_core::scan;
