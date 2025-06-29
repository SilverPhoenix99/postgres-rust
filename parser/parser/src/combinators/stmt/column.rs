pub(super) fn column(stream: &mut TokenStream) -> scan::Result<QualifiedName> {
    /*
        COLUMN any_name
    */

    let (_, name) = (Column, any_name)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_column() {
        test_parser!(
            source = "column foo",
            parser = column,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Column;
