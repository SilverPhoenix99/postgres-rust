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

    /// Returns the result from both parsers, in order, or the first `Err`.
    ///
    /// This is equivalent to `Self & R`.
    fn and<R>(self, right: R) -> impl Combinator<Output = (Self::Output, R::Output)>
    where
        R: Combinator
    {
        (self, right)
    }

    /// Same as `(Self && R)`
    fn and_then<R, M, O>(self, right: R, mapper: M) -> impl Combinator<Output = O>
    where
        R: Combinator,
        M: Fn(Self::Output, R::Output) -> O
    {
        self.and(right)
            .map(move |(left, right)| mapper(left, right))
    }

    /// Same as `(Self && R)`.
    ///
    /// Returns `Right::Output`.
    fn and_right<R>(self, right: R) -> impl Combinator<Output = R::Output>
    where
        R: Combinator
    {
        self.and_then(right, |_, right| right)
    }

    /// See [`map()`](map::map).
    fn map<M, O>(self, mapper: M) -> impl Combinator<Output = O>
    where
        M: Fn(Self::Output) -> O
    {
        map(self, mapper)
    }

    /// See [`map_result()`](map_result).
    fn map_result<M, O>(self, mapper: M) -> impl Combinator<Output = O>
    where
        M: Fn(scan::Result<Self::Output>) -> scan::Result<O>
    {
        map_result(self, mapper)
    }

    fn right<L, R>(self) -> impl Combinator<Output = R>
    where
        Self: Combinator<Output = (L, R)>
    {
        self.map(|(_, right)| right)
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

macro_rules! tuple_combinator {
    ($($t:ident => $f:tt),+) => {
        /// Joins multiple parsers into a single parser,
        /// and where the returned parser returns the first `Err`.
        ///
        /// If all parsers return `Ok`, then a tuple with all results is returned.
        ///
        /// Equivalent to `A & B ( & ... )*`.
        impl<T0, $($t),+> Combinator for (T0, $($t),+)
        where
            T0: Combinator,
            $($t: Combinator),+
        {
            type Output = (T0::Output, $($t::Output),+);

            fn parse(&self, stream: &mut TokenStream) -> scan::Result<Self::Output> {
                Ok((
                    self.0.parse(stream)?,
                    $(self.$f.parse(stream).required()?),+
                ))
            }
        }
    };
}

tuple_combinator!(T1 => 1);
tuple_combinator!(T1 => 1, T2 => 2);
tuple_combinator!(T1 => 1, T2 => 2, T3 => 3);
tuple_combinator!(T1 => 1, T2 => 2, T3 => 3, T4 => 4);
tuple_combinator!(T1 => 1, T2 => 2, T3 => 3, T4 => 4, T5 => 5);

use crate::combinators::foundation::map;
use crate::combinators::foundation::map_result;
use crate::combinators::foundation::optional;
use crate::result::Required;
use crate::scan;
use crate::stream::TokenStream;
