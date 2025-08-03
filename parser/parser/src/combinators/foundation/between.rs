/// Matches `'(' P ')'`, and returns the result of `P`, discarding both Parenthesis tokens.
pub(in crate::combinators) fn paren<P>(parser: P) -> impl Combinator<Output = P::Output>
where
    P: Combinator
{
    let between = seq!(OpenParenthesis, parser, CloseParenthesis);
    foundation::parser(move |stream| {
        let (_, value, _) = between.parse(stream)?;
        Ok(value)
    })
}

/// Matches `'[' P ']'`, and returns the result of `P`, discarding both Bracket tokens.
pub(in crate::combinators) fn brackets<P>(parser: P) -> impl Combinator<Output = P::Output>
where
    P: Combinator
{
    let between = seq!(OpenBracket, parser, CloseBracket);
    foundation::parser(move |stream| {
        let (_, value, _) = between.parse(stream)?;
        Ok(value)
    })
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
            parser = paren(integer),
            expected = NonNegative::from(1u32)
        )
    }

    #[test]
    fn test_between_brackets() {
        test_parser!(
            source = "[1]",
            parser = brackets(integer),
            expected = NonNegative::from(1u32)
        )
    }
}

use crate::combinators::foundation;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use pg_lexer::OperatorKind::CloseBracket;
use pg_lexer::OperatorKind::CloseParenthesis;
use pg_lexer::OperatorKind::OpenBracket;
use pg_lexer::OperatorKind::OpenParenthesis;
