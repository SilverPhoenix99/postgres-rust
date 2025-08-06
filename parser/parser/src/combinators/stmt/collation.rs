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
    use pg_combinators::test_parser;

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
use pg_basics::QualifiedName;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Collation;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
