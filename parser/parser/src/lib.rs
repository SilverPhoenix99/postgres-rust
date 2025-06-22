extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate core;

mod combinators;
mod config;
mod error;
mod parser;
mod result;
mod stream;
mod tests;

mod eof {
    pub(crate) use crate::error::eof::error::*;
    pub(crate) use crate::error::eof::result::*;
}

mod scan {
    pub(crate) use crate::error::scan::error::*;
    pub(crate) use crate::error::scan::result::*;
}

pub use self::{
    config::ParserConfig,
    parser::{Parser, ParserResult}
};
