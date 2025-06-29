pub(in crate::combinators) fn between_paren<P>(parser: P) -> impl Combinator<Output = P::Output>
where
    P: Combinator
{
    (OpenParenthesis, parser, CloseParenthesis)
        .map(|(_, value, _)| value)
}

pub(in crate::combinators) fn between_square<P>(parser: P) -> impl Combinator<Output = P::Output>
where
    P: Combinator
{
    (OpenBracket, parser, CloseBracket)
        .map(|(_, value, _)| value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::foundation::integer;
    use crate::tests::test_parser;
    use pg_basics::NonNegative;

    #[test]
    fn test_between_paren() {
        test_parser!(
            source = "(1)",
            parser = between_paren(integer),
            expected = NonNegative::from(1u32)
        )
    }

    #[test]
    fn test_between_brackets() {
        test_parser!(
            source = "[1]",
            parser = between_square(integer),
            expected = NonNegative::from(1u32)
        )
    }
}

use crate::combinators::foundation::Combinator;
use pg_lexer::OperatorKind::CloseBracket;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::OpenBracket;
use pg_lexer::OperatorKind::OpenParenthesis;
