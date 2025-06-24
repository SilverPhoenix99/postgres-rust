pub(super) fn between_paren<T: Combinator>(combinator: T) -> impl Combinator<Output = T::Output> {
    between(OpenParenthesis, combinator, CloseParenthesis)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::const_numeric::signed_i32_literal;
    use crate::tests::test_parser;

    #[test]
    fn test_between_paren() {
        test_parser!(
            source = "(1)",
            parser = between_paren(signed_i32_literal),
            expected = 1
        )
    }
}

use crate::combinators::foundation::between;
use crate::combinators::foundation::Combinator;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::OpenParenthesis;
