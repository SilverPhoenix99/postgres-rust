pub(crate) trait Combinator
where
    Self: Sized,
{
    type Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output>;

    /// See [`optional()`](optional::optional).
    fn optional(self) -> impl Combinator<Output = Option<Self::Output>> {
        optional(self)
    }

    /// See [`map()`](map::map).
    fn map<M, O>(self, mapper: M) -> impl Combinator<Output = O>
    where
        M: Fn(Self::Output) -> O
    {
        map(self, mapper)
    }

    fn skip(self) -> impl Combinator<Output = ()> {
        self.map(|_| ())
    }
}

impl<F, O> Combinator for F
where
    F: for<'a> Fn(&'a mut TokenStream<'_>) -> scan::Result<O>,
{
    type Output = O;

    fn parse(&self, stream: &mut TokenStream<'_>) -> scan::Result<Self::Output> {
        self(stream)
    }
}

use crate::combinators::foundation::map;
use crate::combinators::foundation::optional;
use crate::scan;
use crate::stream::TokenStream;
