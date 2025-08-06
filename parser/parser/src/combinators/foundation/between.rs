/// Matches `'(' P ')'`, and returns the result of `P`, discarding both Parenthesis tokens.
macro_rules! paren {
    ($parser:expr) => {
        pg_combinators::parser(|stream| {

            let p = $crate::combinators::foundation::seq!(
                pg_lexer::OperatorKind::OpenParenthesis,
                $parser,
                pg_lexer::OperatorKind::CloseParenthesis
            );

            let (_, value, _) = pg_combinators::Combinator::parse(&p, stream)?;
            Ok(value)
        })
    };
}

/// Matches `'[' P ']'`, and returns the result of `P`, discarding both Bracket tokens.
macro_rules! brackets {
    ($parser:expr) => {
        pg_combinators::parser(|stream| {

            let p = $crate::combinators::foundation::seq!(
                pg_lexer::OperatorKind::OpenBracket,
                $parser,
                pg_lexer::OperatorKind::CloseBracket
            );

            let (_, value, _) = pg_combinators::Combinator::parse(&p, stream)?;
            Ok(value)
        })
    };
}

pub(in crate::combinators) use {brackets, paren};

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
            parser = paren!(integer),
            expected = NonNegative::from(1u32)
        )
    }

    #[test]
    fn test_between_brackets() {
        test_parser!(
            source = "[1]",
            parser = brackets!(integer),
            expected = NonNegative::from(1u32)
        )
    }
}
