pub(super) fn collation(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        COLLATION any_name
    */

    let (_, name) = seq!(Collation, any_name)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Collation;
