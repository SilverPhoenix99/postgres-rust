pub(in crate::combinators::stmt) fn statistics(ctx: &mut ParserContext) -> scan::Result<QualifiedName> {

    /*
        STATISTICS any_name
    */

    let (_, name) = seq!(Statistics, any_name)
        .parse(ctx)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;

    #[test]
    fn test_statistics() {
        test_parser!(
            source = "statistics foo",
            parser = statistics,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Statistics;
use pg_parser_core::scan;
