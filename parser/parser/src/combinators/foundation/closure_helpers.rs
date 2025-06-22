pub(in crate::combinators) trait ClosureHelpers<O>
where
    Self: Sized + Fn(&mut TokenStream) -> Result<O>
{
    fn parse(&self, stream: &mut TokenStream) -> Result<O> {
        self(stream)
    }

    fn optional(self) -> impl Combinator<Output = Option<O>> {
        parser(self).optional()
    }

    fn required(self) -> impl Combinator<Output = O> {
        parser(self).required()
    }

    fn try_match(self) -> impl Combinator<Output = Option<O>> {
        parser(self).try_match()
    }

    fn maybe_match(self) -> impl Combinator<Output = Option<O>> {
        parser(self).maybe_match()
    }

    fn and<R>(self, right: R) -> impl Combinator<Output = (O, R::Output)>
    where
        R: Combinator
    {
        parser(self).and(right)
    }

    fn and_then<R, M, P>(self, right: R, mapper: M) -> impl Combinator<Output = P>
    where
        R: Combinator,
        M: Fn(O, R::Output) -> P
    {
        parser(self).and_then(right, mapper)
    }

    fn and_left<R>(self, right: R) -> impl Combinator<Output = O>
    where
        R: Combinator
    {
        parser(self).and_left(right)
    }

    fn and_right<R>(self, right: R) -> impl Combinator<Output = R::Output>
    where
        R: Combinator
    {
        parser(self).and_right(right)
    }

    fn or<R>(self, right: R) -> impl Combinator<Output = O>
    where
        R: Combinator<Output = O>
    {
        parser(self).or(right)
    }

    fn map<M, P>(self, mapper: M) -> impl Combinator<Output = P>
    where
        M: Fn(O) -> P
    {
        parser(self).map(mapper)
    }

    fn map_err<M>(self, mapper: M) -> impl Combinator<Output = O>
    where
        M: Fn(Error) -> Error
    {
        parser(self).map_err(mapper)
    }
    
    fn map_result<M, P>(self, mapper: M) -> impl Combinator<Output = P>
    where
        M: Fn(Result<O>) -> Result<P>
    {
        parser(self).map_result(mapper)
    }

    fn skip(self) -> impl Combinator<Output = ()> {
        parser(self).skip()
    }

    fn chain_result<M, P>(self, mapper: M) -> impl Combinator<Output = P>
    where
        M: Fn(Result<O>, &mut TokenStream) -> Result<P>
    {
        parser(self).chain_result(mapper)
    }

    fn chain<M, P>(self, mapper: M) -> impl Combinator<Output = P>
    where
        M: Fn(O, &mut TokenStream) -> Result<P>
    {
        parser(self).chain(mapper)
    }

    fn chain_err<M>(self, mapper: M) -> impl Combinator<Output = O>
    where
        M: Fn(Error, &mut TokenStream) -> Result<O>
    {
        parser(self).chain_err(mapper)
    }
}

impl<T, O> ClosureHelpers<O> for T
where
    T: Sized + Fn(&mut TokenStream) -> Result<O>
{}

use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan::{Error, Result};
use crate::stream::TokenStream;
