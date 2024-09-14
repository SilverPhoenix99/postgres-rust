mod basic_decoder;
mod bit_decoder;
mod complex_decoder;
mod extended_decoder;
mod unicode_decoder;

pub use basic_decoder::BasicStringDecoder;
pub use bit_decoder::{BitStringDecoder, BitStringError};
pub use extended_decoder::{ExtendedStringDecoder, ExtendedStringError};
pub use unicode_decoder::{UnicodeStringDecoder, UnicodeStringError};

#[inline(always)]
pub fn decode_char(c: u8, radix: u32) -> Option<u32> {
    (c as char).to_digit(radix)
}
