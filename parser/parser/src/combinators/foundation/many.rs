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

            let element = match $prefix {
                Ok(ok) => ok,
                Err(err) => break 'block Err(err)
            };

            let mut elements = vec![element];

            while let Some(element) = {
                match $combinator.optional() {
                    Ok(ok) => ok,
                    Err(err) => break 'block Err(err.into())
                }
            } {
                elements.push(element)
            }

            Ok(elements)
        }
    };

    (=> sep = $separator:expr, $combinator:expr) => {
        'block: {
            use $crate::result::{Optional, Required};

            let element = match $combinator {
                Ok(ok) => ok,
                Err(err) => break 'block Err(err)
            };
            let mut elements = vec![element];

            while {
                    match $separator.optional() {
                        Ok(ok) => ok.is_some(),
                        Err(err) => break 'block Err(err.into())
                    }
                }
            {
                let element = match $combinator.required() {
                    Ok(ok) => ok,
                    Err(err) => break 'block Err(err.into())
                };
                elements.push(element);
            }

            Ok(elements)
        }
    };

    (=> $combinator:expr) => {
        many!(=> pre = $combinator, $combinator)
    };

    (pre = $prefix:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            many!(=> pre = $prefix.parse(stream), $combinator.parse(stream))
        })
    };

    (sep = $separator:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            many!(=> sep = $separator.parse(stream), $combinator.parse(stream))
        })
    };

    ($combinator:expr) => {
        $crate::combinators::foundation::parser(|stream| {
            many!(=> $combinator.parse(stream))
        })
    };
}

pub(in crate::combinators) use many;
