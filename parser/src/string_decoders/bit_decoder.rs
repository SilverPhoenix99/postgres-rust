use bitvec::boxed::BitBox;
use bitvec::vec::BitVec;
use BitStringError::{BitStringTooLong, InvalidBinaryDigit};

const BITS_PER_BYTE: usize = 8;
const VARBITMAXLEN: usize = i32::MAX as usize - BITS_PER_BYTE + 1;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitStringError {
    BitStringTooLong,
    InvalidBinaryDigit,
}

pub struct BitStringDecoder<'src> {
    source: &'src [u8],
    bits_per_char: usize
}

impl<'src> BitStringDecoder<'src> {

    #[inline(always)]
    pub fn new(source: &'src [u8], is_hex: bool) -> Self {
        let bits_per_char = if is_hex { 4 } else { 1 };
        Self { source, bits_per_char }
    }

    pub fn decode(&self) -> Result<BitBox, BitStringError> {

        // see [bit_in](https://github.com/postgres/postgres/blob/77761ee5dddc0518235a51c533893e81e5f375b9/src/backend/utils/adt/varbit.c#L147)

        let radix = 1 << self.bits_per_char;

        let chars_per_byte = BITS_PER_BYTE / self.bits_per_char;
        if self.source.len() > VARBITMAXLEN / self.bits_per_char {
            return Err(BitStringTooLong);
        }

        let src = self.source.iter()
            .map(|c|
                 (*c as char)
                     .to_digit(radix)
                     .ok_or(InvalidBinaryDigit)
            );

        let buffer = if self.bits_per_char == 1 {
            src.map(|bit| bit.map(|b| b == 1))
                .collect::<Result<BitVec, _>>()?
        }
        else {
            let capacity = src.len() / chars_per_byte + 1;
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
        let decoder = BitStringDecoder::new(b"01011010", false);
        let expected = bitvec![usize, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0];
        assert_eq!(
            Ok(expected.into_boxed_bitslice()),
            decoder.decode()
        )
    }

    #[test]
    fn test_hex_string() {
        let decoder = BitStringDecoder::new(b"5a", true);
        let expected = bitvec![usize, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0];
        assert_eq!(
            Ok(expected.into_boxed_bitslice()),
            decoder.decode()
        )
    }
}
