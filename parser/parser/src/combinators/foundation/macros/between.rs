macro_rules! between {

    (paren : $stream:ident => $content:expr) => {{
        $crate::combinators::foundation::between!($stream =>
            {
                let p = $crate::combinators::foundation::Combinator::skip(pg_lexer::OperatorKind::OpenParenthesis);
                $crate::combinators::foundation::Combinator::parse(&p, $stream)
            },
            $content,
            {
                let p = $crate::combinators::foundation::Combinator::skip(pg_lexer::OperatorKind::CloseParenthesis);
                $crate::combinators::foundation::Combinator::parse(&p, $stream)
            }
        )
    }};

    (square : $stream:ident => $content:expr) => {{
        $crate::combinators::foundation::between!($stream =>
            {
                let p = $crate::combinators::foundation::Combinator::skip(pg_lexer::OperatorKind::OpenBracket);
                $crate::combinators::foundation::Combinator::parse(&p, $stream)
            },
            $content,
            {
                let p = $crate::combinators::foundation::Combinator::skip(pg_lexer::OperatorKind::CloseBracket);
                $crate::combinators::foundation::Combinator::parse(&p, $stream)
            }
        )
    }};

    ($stream:ident => $before:expr, $content:expr, $after:expr) => {{
        let result = $crate::combinators::foundation::seq!(=> $before, $content, $after);
        result.map(|(_, content, _)| content)
    }};
}

pub(in crate::combinators) use between;

#[cfg(test)]
mod tests {
    use pg_basics::NonNegative;
    use crate::combinators::foundation::integer;
    use crate::combinators::foundation::parser;
    use crate::tests::test_parser;
    use super::*;

    #[test]
    fn test_between_paren() {
        test_parser!(
            source = "(1)",
            parser = parser(|stream|
                between!(paren : stream => integer(stream))
            ),
            expected = NonNegative::from(1u32)
        )
    }

    #[test]
    fn test_between_brackets() {
        test_parser!(
            source = "[1]",
            parser = parser(|stream|
                between!(square : stream => integer(stream))
            ),
            expected = NonNegative::from(1u32)
        )
    }
}
