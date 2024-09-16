mod unicode_string_error;

use postgres_basics::UnicodeChar::{SurrogateFirst, SurrogateSecond};
use postgres_basics::UnicodeCharError::*;
use postgres_basics::{wchar, CharBuffer, UnicodeChar};
pub use unicode_string_error::UnicodeStringError;
use UnicodeStringError::*;

pub struct UnicodeStringDecoder<'src> {
    input: CharBuffer<'src>,
    quote: u8,
    escape: u8,
}

impl<'src> UnicodeStringDecoder<'src> {

    pub fn new(source: &'src [u8], is_ident: bool, escape: u8) -> Self {
        let input = CharBuffer::new(source);
        let quote = if is_ident { b'"' } else { b'\'' };
        Self { input, quote, escape }
    }

    pub fn decode(&mut self) -> Result<String, UnicodeStringError> {

        // see [str_udeescape](https://github.com/postgres/postgres/blob/1c61fd8b527954f0ec522e5e60a11ce82628b681/src/backend/parser/parser.c#L372)

        // Quotes are escaped by duplicating themselves.
        // '' -> '
        // "" -> "

        let mut out = Vec::with_capacity(self.input.source().len());

        while let Some(c) = self.input.consume_one() {

            if c == self.quote {
                out.push(c);

                // ignore duplicate quote char (escapes itself: '' or "")
                self.input.consume_char(self.quote);

                continue
            }

            if c != self.escape {
                out.push(c);
                continue
            }

            let start_index = self.input.current_index();
            let unicode_len = if self.input.consume_char(b'+') { 6 } else { 4 };

            let c = match self.input.consume_unicode_char(unicode_len) {
                Ok(UnicodeChar::Utf8(c)) => c,
                Ok(SurrogateFirst(first)) => {
                    if !self.input.consume_char(self.escape) {
                        return Err(InvalidUnicodeSurrogatePair)
                    }

                    let unicode_len = if self.input.consume_char(b'+') { 6 } else { 4 };
                    if let Ok(SurrogateSecond(second)) = self.input.consume_unicode_char(unicode_len) {
                        wchar::decode_utf16(first, second)
                            .ok_or(InvalidUnicodeSurrogatePair)? // should be unreachable
                    } else {
                        return Err(InvalidUnicodeSurrogatePair)
                    }
                }
                Err(LenTooShort { .. }) => {
                    // It wasn't a valid Unicode escape.
                    // Just push all chars up to here

                    out.push(self.escape);

                    let end_index = self.input.current_index();
                    let src = &self.input.source()[start_index..end_index];
                    out.extend_from_slice(src);
                    continue
                },
                Ok(SurrogateSecond(_)) => return Err(InvalidUnicodeSurrogatePair),
                Err(InvalidUnicodeEscape) => return Err(InvalidUnicodeCodepoint),
            };

            let len = c.len_utf8();
            if len == 1 {
                // fast path
                out.push(c as u8);
                continue
            }

            // Avoid allocating a string, by encoding the char directly,
            // and pushing the raw bytes directly into the output buffer.
            let mut buff = [0; 4];
            c.encode_utf8(&mut buff);
            out.extend_from_slice(&buff[..len]);
        }

        String::from_utf8(out)
            .map_err(|err| Utf8(err.utf8_error()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_string() {
        let mut decoder = UnicodeStringDecoder::new(b"''d!0061t\\+000061 a!b \\", false, b'!');
        assert_eq!(
            Ok("'dat\\+000061 a!b \\".to_string()),
            decoder.decode()
        )
    }
}
