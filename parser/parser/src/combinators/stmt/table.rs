pub(super) fn table(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        TABLE any_name
    */

    let (_, name) = seq!(Table, any_name)
        .parse(stream)?;

    Ok(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

    #[test]
    fn test_table() {
        test_parser!(
            source = "table foo",
            parser = table,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::Table;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
