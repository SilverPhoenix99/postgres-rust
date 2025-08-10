/// Matches `'(' P ')'`, and returns the result of `P`, discarding both Parenthesis tokens.
#[macro_export]
macro_rules! paren {
    ($parser:expr) => {
        $crate::parser(|ctx| {

            let p = $crate::seq!(
                pg_lexer::OperatorKind::OpenParenthesis,
                $parser,
                pg_lexer::OperatorKind::CloseParenthesis
            );

            let (_, value, _) = $crate::Combinator::parse(&p, ctx)?;
            Ok(value)
        })
    };
}

/// Matches `'[' P ']'`, and returns the result of `P`, discarding both Bracket tokens.
#[macro_export]
macro_rules! brackets {
    ($parser:expr) => {
        $crate::parser(|ctx| {

            let p = $crate::seq!(
                pg_lexer::OperatorKind::OpenBracket,
                $parser,
                pg_lexer::OperatorKind::CloseBracket
            );

            let (_, value, _) = $crate::Combinator::parse(&p, ctx)?;
            Ok(value)
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::integer;
    use crate::test_parser;
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
