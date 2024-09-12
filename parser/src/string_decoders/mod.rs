mod basic_string_decoder;
mod bit_string_decoder;

pub use basic_string_decoder::BasicStringDecoder;
pub use bit_string_decoder::{BitStringDecoder, BitStringError};

#[inline(always)]
pub fn decode_char(c: u8, radix: u32) -> Option<u32> {
    (c as char).to_digit(radix)
}
