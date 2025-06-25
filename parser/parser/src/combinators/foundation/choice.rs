macro_rules! choice {

    ($stream:expr =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        'block: {
            match $crate::result::MaybeMatch::maybe_match($head) {
                Ok(Some(ok)) => break 'block Ok(ok),
                Err(err) => break 'block Err($crate::scan::Error::from(err)),
                Ok(None) => {}
            }

            $(
                match $crate::result::MaybeMatch::maybe_match($tail) {
                    Ok(Some(ok)) => break 'block Ok(ok),
                    Err(err) => break 'block Err($crate::scan::Error::from(err)),
                    Ok(None) => {}
                }
            )+

            let loc = $crate::stream::TokenStream::current_location($stream);
            Err($crate::scan::Error::NoMatch(loc))
        }
    };

    (parsed $stream:expr =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        choice!($stream =>
            $crate::combinators::foundation::Combinator::parse(&$head, $stream),
            $(
                $crate::combinators::foundation::Combinator::parse(&$tail, $stream)
            ),+
        )
    };

    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            choice!(parsed stream =>
                $crate::combinators::foundation::Combinator::map($head, From::from),
                $(
                    $crate::combinators::foundation::Combinator::map($tail, From::from)
                ),+
            )
        })
    };
}

macro_rules! seq {

    (=>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        'block: {
            let value = (
                match $head {
                    Ok(ok) => ok,
                    Err(err) => break 'block Err($crate::scan::Error::from(err)),
                },
                $(
                    match {
                        let result = $tail;
                        let result = result.map_err($crate::scan::Error::from);
                        $crate::result::Required::required(result)
                    } {
                        Ok(ok) => ok,
                        Err(err) => break 'block Err($crate::scan::Error::from(err)),
                    }
                ),+
            );

            Ok(value)
        }
    };

    ($stream:expr =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        seq!(=>
            $crate::combinators::foundation::Combinator::parse(&$head, $stream),
            $(
                $crate::combinators::foundation::Combinator::parse(&$tail, $stream)
            ),+
        )
    };
}

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

pub(in crate::combinators) use {between, choice, seq};
