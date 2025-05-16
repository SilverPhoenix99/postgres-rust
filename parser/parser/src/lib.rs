#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod combinators;
mod config;
mod parser;
mod result;
mod stream;
mod tests;

pub use self::{
    config::ParserConfig,
    parser::{Parser, ParserResult}
};
