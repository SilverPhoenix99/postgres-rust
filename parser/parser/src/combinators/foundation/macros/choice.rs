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

pub(in crate::combinators) use choice;
