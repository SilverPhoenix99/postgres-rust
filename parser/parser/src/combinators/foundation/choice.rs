macro_rules! choice {

    ($stream:expr =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        'block: {
            use $crate::result::MaybeMatch;
            use $crate::scan::Error;

            match $head.maybe_match() {
                Ok(Some(ok)) => break 'block Ok(ok),
                Err(err) => break 'block Err(Error::from(err)),
                Ok(None) => {}
            }

            $(
                match $tail.maybe_match() {
                    Ok(Some(ok)) => break 'block Ok(ok),
                    Err(err) => break 'block Err(Error::from(err)),
                    Ok(None) => {}
                }
            )+

            let loc = $stream.current_location();
            Err(Error::NoMatch(loc))
        }
    };

    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            choice!(stream =>
                $head.parse(stream).map(From::from),
                $(
                    $tail.parse(stream).map(From::from)
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
    ) => {
        seq!(=>
            $head.parse($stream),
            $(
                $tail.parse($stream)
            ),+
        )
    };
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
