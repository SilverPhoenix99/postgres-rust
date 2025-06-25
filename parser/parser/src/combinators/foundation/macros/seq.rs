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

pub(in crate::combinators) use seq;
