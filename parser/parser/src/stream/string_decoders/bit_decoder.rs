const BITS_PER_BYTE: usize = 8;
pub const VARBITMAXLEN: usize = i32::MAX as usize - BITS_PER_BYTE + 1;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(in crate::stream) enum BitStringError {
    BitStringTooLong,
    InvalidBinaryDigit,
}

pub(in crate::stream) struct BitStringDecoder<'src> {
    source: &'src str,
    bits_per_char: u8
}

impl<'src> BitStringDecoder<'src> {

    #[inline(always)]
    pub fn new(source: &'src str, is_hex: bool) -> Self {
        let bits_per_char = if is_hex { 4 } else { 1 };
        Self { source, bits_per_char }
    }

    pub fn decode(&self) -> Result<BitBox, BitStringError> {

        // see [bit_in](https://github.com/postgres/postgres/blob/77761ee5dddc0518235a51c533893e81e5f375b9/src/backend/utils/adt/varbit.c#L147)

        let radix = 1 << self.bits_per_char;

        let chars_per_byte = BITS_PER_BYTE / self.bits_per_char as usize;
        if self.source.len() > VARBITMAXLEN / self.bits_per_char as usize {
            return Err(BitStringTooLong);
        }

        let src = self.source.chars()
            .map(|c| c.to_digit(radix).ok_or(InvalidBinaryDigit));

        let buffer = if self.bits_per_char == 1 {
            src.map(|bit| bit.map(|b| b == 1))
                .collect::<Result<BitVec, _>>()?
        }
        else {
            let capacity = self.source.len() / chars_per_byte + 1;
            let mut buffer = BitVec::with_capacity(capacity);

            for bits in src {
                let bits = bits?;
                let bits = (0..self.bits_per_char).rev()
                    .map(|i| ((bits >> i) & 1) == 1);
                bits.for_each(|b| buffer.push(b));
            }

            buffer
        };

        Ok(buffer.into_boxed_bitslice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::bitvec;
    use bitvec::order::Lsb0;

    #[test]
    fn test_binary_string() {
        let decoder = BitStringDecoder::new("01011010", false);
        let expected = bitvec![usize, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0];
        assert_eq!(
            Ok(expected.into_boxed_bitslice()),
            decoder.decode()
        )
    }

    #[test]
    fn test_hex_string() {
        let decoder = BitStringDecoder::new("5a", true);
        let expected = bitvec![usize, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0];
        assert_eq!(
            Ok(expected.into_boxed_bitslice()),
            decoder.decode()
        )
    }
}

use bitvec::boxed::BitBox;
use bitvec::vec::BitVec;
use BitStringError::{BitStringTooLong, InvalidBinaryDigit};
