pub(super) fn view(stream: &mut TokenStream) -> scan::Result<QualifiedName> {

    /*
        VIEW any_name
    */

    let (_, view) = seq!(stream => View, any_name)?;

    Ok(view)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;

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
use crate::combinators::foundation::seq;
use crate::scan;
use crate::stream::TokenStream;
use pg_basics::QualifiedName;
use pg_lexer::Keyword::View;
