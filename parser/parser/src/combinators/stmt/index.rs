pub(super) fn index(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        INDEX any_name
    */

    let (_, name) = seq!(Index, any_name)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Index;
