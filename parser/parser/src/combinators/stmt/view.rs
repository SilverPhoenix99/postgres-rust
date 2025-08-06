pub(super) fn view(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        VIEW any_name
    */

    let (_, view) = seq!(View, any_name).parse(stream)?;

    Ok(view)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;

    #[test]
    fn test_view() {
        test_parser!(
            source = "view foo",
            parser = view,
            expected = vec!["foo".into()]
        )
    }
}

use crate::combinators::any_name;
use pg_basics::QualifiedName;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::View;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
