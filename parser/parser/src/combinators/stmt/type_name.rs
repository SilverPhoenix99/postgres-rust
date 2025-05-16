pub(super) fn type_name() -> impl Combinator<Output = Type> {

    Kw::Type
        .and_right(typename())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use pg_ast::TypeName::Int4;

    #[test]
    fn test_type() {
        test_parser!(
            source = "type int",
            parser = type_name(),
            expected = Int4.into()
        )
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::typename;
use pg_ast::Type;
use pg_lexer::Keyword as Kw;
