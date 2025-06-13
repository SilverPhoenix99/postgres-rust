pub(crate) trait Combinator: Debug {
    type Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output>;
}

use crate::scan::Result;
use crate::stream::TokenStream;
use std::fmt::Debug;
