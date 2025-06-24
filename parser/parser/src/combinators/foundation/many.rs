/// # `many!(P)`
///
/// Does `( P )+`.
///
/// ## Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
///
/// # `many!(pre = P, Q)`
///
/// Does `P ( Q )*`, where both `P` and `Q` are the same type, with different parsers.
///
/// If `P` and `Q` are the same parser, then use `many!(P)`.
///
/// ## Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
///
/// # `many!(sep = S, P)`
///
/// Does `P ( S P )*`, i.e., `P` separated by `S`.
///
/// `S` is discarded.
///
/// To do `P ( S Q )*`, where `P` and `Q` are different parsers returning the same type,
/// then use the following, where `S` needs to be discarded in the `follow` parser:
///
/// ```many!(pre = P, seq!(S, Q))```
///
/// ## Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
macro_rules! many {

    (=> pre = $prefix:expr, $combinator:expr) => {
        'block: {
            use $crate::result::Optional;
            use $crate::scan::Error;

            let element = match $prefix.map_err(Error::from) {
                Ok(ok) => ok,
                Err(err) => break 'block Err(err)
            };

            let mut elements = vec![element];

            while let Some(element) = {
                match $combinator.optional().map_err(Error::from) {
                    Ok(ok) => ok,
                    Err(err) => break 'block Err(err.into())
                }
            } {
                elements.push(element)
            }

            break 'block Ok(elements)
        }
    };

    (=> sep = $separator:expr, $combinator:expr) => {
        'block: {
            use $crate::result::{Optional, Required};
            use $crate::scan::Error;

            let element = match $combinator.map_err(Error::from) {
                Ok(ok) => ok,
                Err(err) => break 'block Err(err)
            };
            let mut elements = vec![element];

            while {
                    match $separator.optional().map_err(Error::from) {
                        Ok(ok) => ok.is_some(),
                        Err(err) => break 'block Err(err)
                    }
                }
            {
                let element = match $combinator.required().map_err(Error::from) {
                    Ok(ok) => ok,
                    Err(err) => break 'block Err(err)
                };
                elements.push(element);
            }

            break 'block Ok(elements)
        }
    };

    (=> $combinator:expr) => {
        many!(=> pre = $combinator, $combinator)
    };

    ($stream:expr => pre = $prefix:expr, $combinator:expr) => {{
        use $crate::combinators::foundation::Combinator;
        many!(=>
            pre = $prefix.parse($stream),
            $combinator.parse($stream)
        )
    }};

    ($stream:expr => sep = $separator:expr, $combinator:expr) => {{
        use $crate::combinators::foundation::Combinator;
        many!(=>
            sep = $separator.parse($stream),
            $combinator.parse($stream)
        )
    }};

    ($stream:expr => $combinator:expr) => {{
        use $crate::combinators::foundation::Combinator;
        many!(=> $combinator.parse($stream))
    }};

    (pre = $prefix:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            many!(=> pre = $prefix.parse(stream), $combinator.parse(stream))
        })
    };

    (sep = $separator:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            many!(=> sep = $separator.parse(stream), $combinator.parse(stream))
        })
    };

    ($combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            many!(=> $combinator.parse(stream))
        })
    };
}

pub(in crate::combinators) use many;
