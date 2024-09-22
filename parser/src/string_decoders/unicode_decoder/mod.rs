mod unicode_string_error;

pub use self::unicode_string_error::UnicodeStringError;
use self::unicode_string_error::UnicodeStringError::*;
use postgres_basics::{
    wchar,
    CharBuffer,
    UnicodeChar,
    UnicodeChar::{SurrogateFirst, SurrogateSecond},
    UnicodeCharError,
    UnicodeCharError::LenTooShort
};

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
                // <xus>{xqdouble} | <xui>{xddouble}
                out.push(c);

                // ignore duplicate quote char (escapes itself: '' or "")
                self.input.consume_char(self.quote);

                continue
            }

            if c != self.escape {
                out.push(c);
                continue
            }

            if self.input.consume_char(self.escape) {
                // double escape char, like `\\`, or `!!`
                out.push(self.escape);
                continue
            }

            let c = self.consume_unicode()?;

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

    fn consume_unicode(&mut self) -> Result<char, UnicodeStringError> {

        let start_index = self.input.current_index() - 1; // include `\`
        let unicode_len = if self.input.consume_char(b'+') { 6 } else { 4 };

        let first = match self.input.consume_unicode_char(unicode_len) {
            Ok(UnicodeChar::Utf8(c)) => return Ok(c),
            Ok(SurrogateFirst(first)) => Ok(first),
            Ok(SurrogateSecond(_)) => Err(InvalidUnicodeSurrogatePair(start_index)),
            Err(LenTooShort(_)) => Err(InvalidUnicodeEscape(start_index)),
            Err(UnicodeCharError::InvalidUnicodeValue) => Err(InvalidUnicodeValue(start_index))
        }?;

        let start_index = self.input.current_index();
        let invalid_pair = InvalidUnicodeSurrogatePair(start_index);

        if !self.input.consume_char(self.escape) {
            return Err(invalid_pair)
        }

        let unicode_len = if self.input.consume_char(b'+') { 6 } else { 4 };

        let second = match self.input.consume_unicode_char(unicode_len) {
            Ok(SurrogateSecond(second)) => second,
            _ => return Err(invalid_pair),
        };

        wchar::decode_utf16(first, second).ok_or(invalid_pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_string() {
        let source = br"''d!0061t\+000061 a!!b \";
        let mut decoder = UnicodeStringDecoder::new(source, false, b'!');
        assert_eq!(
            Ok(r"'dat\+000061 a!b \".to_string()),
            decoder.decode()
        )
    }
}
