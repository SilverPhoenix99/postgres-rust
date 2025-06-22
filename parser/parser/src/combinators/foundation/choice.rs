macro_rules! choice {
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
