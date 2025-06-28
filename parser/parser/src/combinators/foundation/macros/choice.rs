macro_rules! choice {

    ($stream:ident =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        'block: {
            match $crate::result::Optional::optional($head) {
                Ok(Some(ok)) => break 'block Ok(ok),
                Err(err) => break 'block Err($crate::scan::Error::from(err)),
                Ok(None) => {}
            }

            $(
                match $crate::result::Optional::optional($tail) {
                    Ok(Some(ok)) => break 'block Ok(ok),
                    Err(err) => break 'block Err($crate::scan::Error::from(err)),
                    Ok(None) => {}
                }
            )+

            let loc = $crate::stream::TokenStream::current_location($stream);
            Err($crate::scan::Error::NoMatch(loc))
        }
    };

    (parsed $stream:ident =>
        $head:expr,
        $($tail:expr),+
        $(,)?
    ) => {
        $crate::combinators::foundation::choice!($stream =>
            $crate::combinators::foundation::Combinator::parse(&$head, $stream),
            $(
                $crate::combinators::foundation::Combinator::parse(&$tail, $stream)
            ),+
        )
    };
}

pub(in crate::combinators) use choice;
