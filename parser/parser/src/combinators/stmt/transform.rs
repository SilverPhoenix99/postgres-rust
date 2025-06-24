pub(super) fn transform(stream: &mut TokenStream) -> Result<Transform> {

    let (_, _, for_type, _, language) = seq!(stream => Kw::Transform, For, typename, Language, col_id)?;

    Ok(Transform::new(for_type, language))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::TypeName::Int4;

    #[test]
    fn test_transform() {
        test_parser!(
            source = "transform for int language plpgsql",
            parser = transform,
            expected = Transform::new(Int4, "plpgsql")
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::seq;
use crate::combinators::typename;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::Transform;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Language;
