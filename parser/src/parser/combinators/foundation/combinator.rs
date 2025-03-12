pub(in crate::parser) trait Combinator: Debug {
    type Output;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output>;
}

use crate::parser::result::ScanResult;
use crate::parser::token_stream::TokenStream;
use std::fmt::Debug;
