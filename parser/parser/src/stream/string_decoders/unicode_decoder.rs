pub(in crate::stream) struct UnicodeStringDecoder<'src> {
    input: CharBuffer<'src>,
    quote: char,
    escape: char,
}

impl<'src> UnicodeStringDecoder<'src> {

    pub fn new(source: &'src str, is_ident: bool, uescape: char) -> Self {
        let input = CharBuffer::new(source);
        let quote = if is_ident { '"' } else { '\'' };
        Self { input, quote, escape: uescape }
    }

    pub fn decode(&mut self) -> Result {

        // see [str_udeescape](https://github.com/postgres/postgres/blob/1c61fd8b527954f0ec522e5e60a11ce82628b681/src/backend/parser/parser.c#L372)

        // Quotes are escaped by duplicating themselves.
        // '' -> '
        // "" -> "

        let mut out = String::with_capacity(self.input.source().len());

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
            out.push(c);
        }

        Ok(out.into_boxed_str())
    }

    fn consume_unicode(&mut self) -> Result<char> {

        let start_index = self.input.current_index() - 1; // include `\`
        let unicode_len = if self.input.consume_char('+') { 6 } else { 4 };

        let first = match self.input.consume_unicode_char(unicode_len) {
            Ok(UnicodeChar::Utf8(c)) => return Ok(c),
            Ok(SurrogateFirst(first)) => Ok(first),
            Ok(SurrogateSecond(_)) => Err(InvalidUnicodeSurrogatePair(start_index)),
            Err(LenTooShort) => Err(InvalidUnicodeEscape(start_index)),
            Err(UnicodeCharError::InvalidUnicodeValue) => Err(InvalidUnicodeValue(start_index)),
        }?;

        let start_index = self.input.current_index();
        let invalid_pair = InvalidUnicodeSurrogatePair(start_index);

        if !self.input.consume_char(self.escape) {
            return Err(invalid_pair)
        }

        let unicode_len = if self.input.consume_char('+') { 6 } else { 4 };

        let Ok(SurrogateSecond(second)) = self.input.consume_unicode_char(unicode_len) else {
            return Err(invalid_pair)
        };

        wchar::decode_utf16(first, second).ok_or(invalid_pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_string() {
        let source = r"''d!0061t\+000061 a!!b \";
        let mut decoder = UnicodeStringDecoder::new(source, false, '!');
        assert_eq!(r"'dat\+000061 a!b \", decoder.decode().unwrap().as_ref())
    }
}

use pg_basics::wchar;
use pg_basics::CharBuffer;
use pg_basics::UnicodeChar;
use pg_basics::UnicodeChar::SurrogateFirst;
use pg_basics::UnicodeChar::SurrogateSecond;
use pg_basics::UnicodeCharError;
use pg_basics::UnicodeCharError::LenTooShort;
use pg_elog::unicode_string::Error::InvalidUnicodeEscape;
use pg_elog::unicode_string::Error::InvalidUnicodeSurrogatePair;
use pg_elog::unicode_string::Error::InvalidUnicodeValue;
use pg_elog::unicode_string::Result;
