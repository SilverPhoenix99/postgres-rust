macro_rules! choice {
    ($stream:expr, $head:expr, $($tail:expr),+ $(,)?) => {{
        (|| -> $crate::scan::Result<_> {
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
    }};
}

pub(in crate::combinators) use choice;
