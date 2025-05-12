pub(super) fn type_name() -> impl Combinator<Output = Type> {

    Kw::Type
        .and_right(typename())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::TypeName::Int4;
    use crate::parser::tests::test_parser;

    #[test]
    fn test_type() {
        test_parser!(
            source = "type int",
            parser = type_name(),
            expected = Int4.into()
        )
    }
}

use crate::parser::ast_node::Type;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::typename;
use postgres_parser_lexer::Keyword as Kw;
