macro_rules! choice {

    (|$stream:ident| {
        $head:expr,
        $($tail:expr),+
        $(,)?
    }) => {
        (|| {
            use $crate::result::MaybeMatch;
            use $crate::scan::Error;

            if let Some(ok) = $head.maybe_match()? {
                return Ok(ok.into())
            }

            $(
                if let Some(ok) = $tail.maybe_match()? {
                    return Ok(ok.into())
                }
            )+

            let loc = $stream.current_location();
            Err(Error::NoMatch(loc))
        })()
    };

    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            use $crate::result::MaybeMatch;
            use $crate::scan::Error;

            if let Some(ok) = $head.parse(stream).maybe_match()? {
                return Ok(ok.into())
            }

            $(
                if let Some(ok) = $tail.parse(stream).maybe_match()? {
                    return Ok(ok.into())
                }
            )+

            let loc = stream.current_location();
            Err(Error::NoMatch(loc))
        })
    };
}

pub(in crate::combinators) use choice;
