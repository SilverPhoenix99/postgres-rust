/// Does `( P )+`.
///
/// # Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
pub(in crate::combinators) fn many<P>(parser: P) -> ManyCombi<P>
where
    P: Combinator
{
    ManyCombi(parser)
}

/// Does `P ( S P )*`, i.e., `P` separated by `S`.
///
/// `S` is discarded.
///
/// To do `P ( S Q )*`, where `P` and `Q` are different parsers returning the same type,
/// then use [`many_m`](many_m) with `pre = P`, where `S` needs to be discarded in the `follow` parser.
///
/// # Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
pub(in crate::combinators) fn many_sep<S, P>(separator: S, parser: P) -> ManySepCombi<S, P>
where
    S: Combinator,
    P: Combinator,
{
    ManySepCombi { parser, separator }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(in crate::combinators) struct ManyCombi<P>(P);

impl<P> Combinator for ManyCombi<P>
where
    P: Combinator
{
    type Output = Vec<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {

        let mut elements = vec![self.0.parse(stream)?];

        while let Some(element) = self.0.parse(stream).optional()? {
            elements.push(element);
        }

        Ok(elements)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! many_m {

    // Does `P ( Q )*`, where both `P` and `Q` are the same type, with different parsers.
    //
    // If `P` and `Q` are the same parser, then use [`many()`](many) to prevent cloning parsers.
    //
    // # Return
    // If `Ok`, then the returned `Vec<_>` is **Never** empty.
    //
    // Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
    (pre = $prefix:expr, $follow:expr $(,)?) => {
        $crate::combinators::foundation::parser(|stream| {
            use $crate::combinators::foundation::Combinator;
            use $crate::result::Optional;

            let first = $prefix.parse(stream)?;
            let mut elements = vec![first];

            let follow = $follow;

            while let Some(element) = follow.parse(stream).optional()? {
                elements.push(element)
            }

            Ok(elements)
        })
    };
}

pub(in crate::combinators) use many_m;

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(in crate::combinators) struct ManySepCombi<S, P> {
    separator: S,
    parser: P,
}

impl<S, P> Combinator for ManySepCombi<S, P>
where
    S: Combinator,
    P: Combinator,
{
    type Output = Vec<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {

        let mut elements = vec![self.parser.parse(stream)?];

        while self.separator.parse(stream).optional()?.is_some() {

            let element = self.parser.parse(stream)
                .required()?;

            elements.push(element);
        }

        Ok(elements)
    }
}

use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::result::Required;
use crate::scan;
use crate::stream::TokenStream;
