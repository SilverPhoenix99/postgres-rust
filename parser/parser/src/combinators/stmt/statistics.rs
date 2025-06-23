pub(super) fn statistics(stream: &mut TokenStream) -> Result<QualifiedName> {

    /*
        STATISTICS any_name
    */

    seq!(stream => Statistics, any_name)
        .map(|(_, name)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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
use crate::combinators::foundation::seq;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Statistics;
