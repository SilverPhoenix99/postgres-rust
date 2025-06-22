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

    (pre = $prefix:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(move |stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::{ClosureHelpers, CombinatorHelpers};
            use $crate::result::Optional;

            let element = $prefix.parse(stream)?;
            let mut elements = vec![element];

            let combinator = $combinator;
            while let Some(element) = combinator.parse(stream).optional()? {
                elements.push(element)
            }

            Ok(elements)
        })
    };

    (sep = $separator:expr, $combinator:expr) => {
        $crate::combinators::foundation::parser(move |stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::{ClosureHelpers, CombinatorHelpers};
            use $crate::result::{Optional, Required};

            let combinator = $combinator;

            let element = combinator.parse(stream)?;
            let mut elements = vec![element];

            let separator = $separator;
            while separator.parse(stream).optional()?.is_some() {
                let element = combinator.parse(stream).required()?;
                elements.push(element);
            }

            Ok(elements)
        })
    };

    ($combinator:expr) => {
        $crate::combinators::foundation::parser(move |stream| {
            #[allow(unused_imports)]
            use $crate::combinators::foundation::{ClosureHelpers, CombinatorHelpers};
            use $crate::result::Optional;

            let combinator = $combinator;

            let element = combinator.parse(stream)?;
            let mut elements = vec![element];

            while let Some(element) = combinator.parse(stream).optional()? {
                elements.push(element);
            }

            Ok(elements)
        })
    };
}

pub(in crate::combinators) use many;
