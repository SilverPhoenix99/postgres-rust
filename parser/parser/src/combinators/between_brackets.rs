pub(super) fn between_brackets<T: Combinator>(combinator: T) -> impl Combinator<Output = T::Output> {
    between(OpenBracket, combinator, CloseBracket)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::signed_i32_literal;
    use crate::tests::test_parser;

    #[test]
    fn test_between_brackets() {
        test_parser!(
            source = "[1]",
            parser = between_brackets(signed_i32_literal),
            expected = 1
        )
    }
}

use crate::combinators::foundation::between;
use crate::combinators::foundation::Combinator;
use pg_lexer::OperatorKind::CloseBracket;
use pg_lexer::OperatorKind::OpenBracket;
