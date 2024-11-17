mod bit_string;
mod identifier;
mod keyword;
mod optional;
mod or;
mod required;
mod string;

pub(in crate::parser) use self::{
    bit_string::bit_string,
    identifier::identifier,
    keyword::{keyword, keyword_if},
    optional::{optional, OptionalCombi},
    required::{required, RequiredCombi},
    string::string,
};
use postgres_basics::FnInfo;

pub(in crate::parser) trait ParserFunc {
    type Output;
    type Error;

    fn parse(&self, stream: &mut TokenStream<'_>) -> Result<Self::Output, Self::Error>;
}

pub(in crate::parser) trait ParserFuncHelpers<T: ParserFunc> {
    fn optional(self) -> OptionalCombi<T>;

    fn required(self, caller: &'static FnInfo) -> RequiredCombi<T>;
}

impl<T: ParserFunc> ParserFuncHelpers<T> for T {

    fn optional(self) -> OptionalCombi<T> {
        optional(self)
    }

    fn required(self, caller: &'static FnInfo) -> RequiredCombi<T> {
        required(self, caller)
    }
}

use crate::parser::token_stream::TokenStream;
