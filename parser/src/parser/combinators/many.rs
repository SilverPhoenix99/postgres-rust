/// Does `( P )+`.
///
/// # Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
pub(in crate::parser) fn many<P>(parser: P) -> ManyCombi<P>
where
    P: Combinator
{
    ManyCombi(parser)
}

/// Does `P ( Q )*`, where both `P` and `Q` are the same type, with different parsers.
///
/// If `P` and `Q` are the same parser, then use [`many()`](many) to prevent cloning parsers.
///
/// # Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
pub(in crate::parser) fn many_pre<P, Q>(first: P, follow: Q) -> ManyPrefixedCombi<P, Q>
where
    P: Combinator,
    Q: Combinator<Output=P::Output>
{
    ManyPrefixedCombi { first, follow }
}

/// Does `P ( S P )*`, i.e., `P` separated by `S`.
///
/// `S` is discarded.
///
/// To do `P ( S Q )*`, where `P` and `Q` are different parsers returning the same type,
/// then use [`many_pre`](many_pre), where `S` needs to be discarded in the `follow` parser.
///
/// # Return
/// If `Ok`, then the returned `Vec<_>` is **Never** empty.
///
/// Returns `Err(NoMatch)` or `Err(Eof)`, if empty.
pub(in crate::parser) fn many_sep<S, P>(separator: S, parser: P) -> ManySepCombi<S, P>
where
    S: Combinator,
    P: Combinator,
{
    ManySepCombi { parser, separator }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(in crate::parser) struct ManyCombi<P>(P);

impl<P> Combinator for ManyCombi<P>
where
    P: Combinator
{
    type Output = Vec<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let mut elements = vec![self.0.parse(stream)?];

        while let Some(element) = self.0.parse(stream).optional()? {
            elements.push(element);
        }

        Ok(elements)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(in crate::parser) struct ManyPrefixedCombi<P, Q> {
    first: P,
    follow: Q
}

impl<P, Q> Combinator for ManyPrefixedCombi<P, Q>
where
    P: Combinator,
    Q: Combinator<Output=P::Output>
{
    type Output = Vec<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let mut elements = vec![self.first.parse(stream)?];

        while let Some(element) = self.follow.parse(stream).optional()? {
            elements.push(element)
        }

        Ok(elements)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(in crate::parser) struct ManySepCombi<S, P> {
    separator: S,
    parser: P,
}

impl<S, P> Combinator for ManySepCombi<S, P>
where
    S: Combinator,
    P: Combinator,
{
    type Output = Vec<P::Output>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let mut elements = vec![self.parser.parse(stream)?];

        while self.separator.parse(stream).optional()?.is_some() {

            let element = self.parser.parse(stream)
                .required()?;

            elements.push(element);
        }

        Ok(elements)
    }
}

use crate::parser::combinators::Combinator;
use crate::parser::result::{Optional, Required, ScanResult};
use crate::parser::token_stream::TokenStream;
