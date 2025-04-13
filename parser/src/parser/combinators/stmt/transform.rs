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
    use crate::parser::ast_node::TypeName::Int4;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_transform() {
        test_parser!(
            source = "transform for int language plpgsql",
            parser = transform(),
            expected = Transform::new(Int4, "plpgsql")
        )
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::For;
use crate::lexer::Keyword::Language;
use crate::parser::ast_node::Transform;
use crate::parser::combinators::col_id;
use crate::parser::combinators::foundation::and;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::typename;
