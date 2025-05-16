#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod string_decoders;

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
