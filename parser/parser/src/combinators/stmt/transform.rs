pub(super) fn transform() -> impl Combinator<Output = Transform> {

    (Kw::Transform, For)
        .and_right(typename())
        .and_then(
            Language.and_right(col_id),
            Transform::new
        )
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
            parser = transform(),
            expected = Transform::new(Int4, "plpgsql")
        )
    }
}

use crate::combinators::col_id;
use crate::combinators::foundation::Combinator;
use crate::combinators::typename;
use pg_ast::Transform;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::For;
use pg_lexer::Keyword::Language;
