use super::decode_char;
use postgres_basics::{wchar, CharBuffer};
use UnicodeChar::*;
use UnicodeCharError::*;

pub(super) enum UnicodeChar {
    SurrogateFirst(u16),
    SurrogateSecond(u16),
    Utf8(char),
}

pub(super) enum UnicodeCharError {
    LenTooShort {
        expected: usize,
        actual: usize,
    },
    InvalidUnicodeEscape,
}

/// This is an internal trait to help decode Unicode chars as sequences of Hex chars.
///
/// E.g.: `'0061'` outputs `'a'`
pub(super) trait ComplexStringDecoder<'src> {
    fn input(&mut self) -> &mut CharBuffer<'src>;

    fn decode_unicode_char(&mut self, unicode_len: usize) -> Result<UnicodeChar, UnicodeCharError> {

        debug_assert!(unicode_len <= 8, "unicode encoded chars cannot be longer than 8 chars");

        let slice = self.input().remainder();
        if slice.len() < unicode_len {
            return Err(LenTooShort {
                expected: unicode_len,
                actual: slice.len(),
            })
        }

        let c = slice[..unicode_len]
            .iter()
            .map(|d| { decode_char(*d, 16).ok_or(InvalidUnicodeEscape) })
            .try_fold(0, |acc, d|
                Ok((acc << 4) + d?)
            )?;

        let result = {
            if wchar::is_utf16_surrogate_first(c as u16) {
                Ok(SurrogateFirst(c as u16))
            } else if wchar::is_utf16_surrogate_second(c as u16) {
                Ok(SurrogateSecond(c as u16))
            }
            else {
                char::from_u32(c)
                    .map(Utf8)
                    .ok_or(InvalidUnicodeEscape)
            }
        };

        if result.is_ok() {
            self.input().consume_many(unicode_len);
        }

        result
    }
}
