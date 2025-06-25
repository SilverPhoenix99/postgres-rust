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
            let element = match $prefix.map_err($crate::scan::Error::from) {
                Ok(ok) => ok,
                Err(err) => break 'block Err(err)
            };

            let mut elements = vec![element];

            while let Some(element) = {
                match {
                    let result = $combinator;
                    let result = $crate::result::Optional::optional(result);
                    result.map_err($crate::scan::Error::from)
                } {
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
            let element = match $combinator.map_err($crate::scan::Error::from) {
                Ok(ok) => ok,
                Err(err) => break 'block Err(err)
            };
            let mut elements = vec![element];

            while {
                    match {
                        let result = $separator;
                        let result = $crate::result::Optional::optional(result);
                        result.map_err($crate::scan::Error::from)
                    } {
                        Ok(ok) => ok.is_some(),
                        Err(err) => break 'block Err(err)
                    }
                }
            {
                let element = match {
                    let result = $combinator;
                    let result = $crate::result::Required::required(result);
                    result.map_err($crate::scan::Error::from)
                } {
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
        let prefix = $prefix;
        let combinator = $combinator;
        many!(=>
            pre = $crate::combinators::foundation::Combinator::parse(&prefix, $stream),
            $crate::combinators::foundation::Combinator::parse(&combinator, $stream)
        )
    }};

    ($stream:expr => sep = $separator:expr, $combinator:expr) => {{
        let separator = $separator;
        let combinator = $combinator;
        many!(=>
            sep = $crate::combinators::foundation::Combinator::parse(&separator, $stream),
            $crate::combinators::foundation::Combinator::parse(&combinator, $stream)
        )
    }};

    ($stream:expr => $combinator:expr) => {{
        let combinator = $combinator;
        many!(=> $crate::combinators::foundation::Combinator::parse(&combinator, $stream))
    }};

    (pre = $prefix:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            many!(stream => pre = $prefix, $combinator)
        })
    };

    (sep = $separator:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            many!(stream => sep = $separator, $combinator)
        })
    };

    ($combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            many!(stream => $combinator)
        })
    };
}

pub(in crate::combinators) use many;
