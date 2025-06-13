#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod combinators;
mod config;
mod parser;
mod result;
mod stream;
mod tests;

mod eof {
    pub(crate) use crate::result::eof::error::*;
    pub(crate) use crate::result::eof::result::*;
}

mod scan {
    pub(crate) use crate::result::scan::error::*;
    pub(crate) use crate::result::scan::result::*;
}

pub use self::{
    config::ParserConfig,
    parser::{Parser, ParserResult}
};
