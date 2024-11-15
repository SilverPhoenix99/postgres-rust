#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod error;
pub mod lexer;
pub mod parser;
pub mod string_decoders;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NumberRadix {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}
