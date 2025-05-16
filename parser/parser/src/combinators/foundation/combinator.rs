pub(crate) trait Combinator: Debug {
    type Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output>;
}

use crate::result::ScanResult;
use crate::stream::TokenStream;
use std::fmt::Debug;
