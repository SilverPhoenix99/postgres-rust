macro_rules! choice {

    ($stream:expr =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        'block: {
            use $crate::scan::Error;

            match $head.map_err(Error::from) {
                Ok(ok) => break 'block Ok(ok),
                Err(Error::NoMatch(_)) => {/* continue */},
                Err(Error::Eof(loc)) => break 'block Err(Error::NoMatch(loc)),
                Err(Error::ScanErr(err)) => break 'block Err(Error::ScanErr(err)),
            }

            $(
                match $tail.map_err(Error::from) {
                    Ok(ok) => break 'block Ok(ok),
                    Err(Error::NoMatch(_)) => {/* continue */},
                    Err(Error::Eof(loc)) => break 'block Err(Error::NoMatch(loc)),
                    Err(Error::ScanErr(err)) => break 'block Err(Error::ScanErr(err)),
                }
            )+

            let loc = $stream.current_location();
            Err(Error::NoMatch(loc))
        }
    };

    (parsed $stream:expr =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        choice!($stream =>
            $head.parse($stream),
            $(
                $tail.parse($stream)
            ),+
        )
    };

    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            choice!(parsed stream =>
                $head.map(From::from),
                $(
                    $tail.map(From::from)
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
            use $crate::result::Required;
            use $crate::scan::Error;

            let value = (
                match $head.map_err(Error::from) {
                    Ok(ok) => ok,
                    Err(err) => break 'block Err(err),
                },
                $(
                    match $tail.map_err(Error::from).required().map_err(Error::from) {
                        Ok(ok) => ok,
                        Err(err) => break 'block Err(err),
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
    ) => {{
        use $crate::combinators::foundation::Combinator;
        seq!(=>
            $head.parse($stream),
            $(
                $tail.parse($stream)
            ),+
        )
    }};
}

macro_rules! between {

    (paren : $stream:expr => $content:expr) => {{
        use $crate::combinators::foundation::Combinator;
        use $crate::combinators::foundation::seq;
        use pg_lexer::OperatorKind::{CloseParenthesis, OpenParenthesis};
        between!($stream =>
            OpenParenthesis.skip().parse($stream),
            $content,
            CloseParenthesis.skip().parse($stream)
        )
    }};

    (square : $stream:expr => $content:expr) => {{
        use $crate::combinators::foundation::Combinator;
        use $crate::combinators::foundation::seq;
        use pg_lexer::OperatorKind::{CloseBracket, OpenBracket};
        between!($stream =>
            OpenBracket.skip().parse($stream),
            $content,
            CloseBracket.skip().parse($stream)
        )
    }};

    ($stream:expr => $before:expr, $content:expr, $after:expr) => {{
        use $crate::combinators::foundation::seq;
        seq!(=> $before, $content, $after)
            .map(|(_, content, _)| content)
    }};

    ($before:expr, $content:expr, $after:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            between!(stream => $before, $content, $after)
        })
    };
}

pub(in crate::combinators) use {between, choice, seq};
