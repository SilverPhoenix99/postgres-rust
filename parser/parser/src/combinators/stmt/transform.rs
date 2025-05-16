pub(super) fn transform() -> impl Combinator<Output = Transform> {

    and(Kw::Transform, For)
        .and_right(typename())
        .and_then(
            Language.and_right(col_id()),
            Transform::new
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use postgres_parser_ast::TypeName::Int4;

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
use crate::combinators::foundation::and;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::typename;
use postgres_parser_ast::Transform;
use postgres_parser_lexer::Keyword as Kw;
use postgres_parser_lexer::Keyword::For;
use postgres_parser_lexer::Keyword::Language;
