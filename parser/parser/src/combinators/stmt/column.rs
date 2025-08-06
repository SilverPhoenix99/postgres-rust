pub(super) fn column(stream: &mut TokenStream) -> scan::Result<QualifiedName> {
    /*
        COLUMN any_name
    */

    let (_, name) = seq!(Column, any_name)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

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
use crate::combinators::foundation::seq;
use pg_basics::QualifiedName;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Column;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
