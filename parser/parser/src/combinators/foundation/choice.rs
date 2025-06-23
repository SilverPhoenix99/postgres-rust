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

            Ok((
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
            ))
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

pub(in crate::combinators) use {choice, seq};
