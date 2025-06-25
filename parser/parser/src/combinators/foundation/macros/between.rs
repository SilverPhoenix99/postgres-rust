macro_rules! between {

    (paren : $stream:expr => $content:expr) => {{
        between!($stream =>
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

    (square : $stream:expr => $content:expr) => {{
        between!($stream =>
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

    ($stream:expr => $before:expr, $content:expr, $after:expr) => {{
        let result = $crate::combinators::foundation::seq!(=> $before, $content, $after);
        result.map(|(_, content, _)| content)
    }};

    ($before:expr, $content:expr, $after:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            between!(stream => $before, $content, $after)
        })
    };
}

pub(in crate::combinators) use between;
