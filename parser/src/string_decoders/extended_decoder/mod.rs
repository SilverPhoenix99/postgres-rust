mod extended_string_error;

pub use extended_string_error::ExtendedStringError;
use postgres_basics::ascii::{is_hex_digit, is_oct_digit};
use postgres_basics::guc::BackslashQuote;
use postgres_basics::guc::BackslashQuote::SafeEncoding;
use postgres_basics::UnicodeChar::{SurrogateFirst, SurrogateSecond};
use postgres_basics::{wchar, CharBuffer, UnicodeChar};
use ExtendedStringError::*;

pub struct ExtendedStringDecoder<'src> {
    input: CharBuffer<'src>,
    backslash_quote: BackslashQuote,
}

impl<'src> ExtendedStringDecoder<'src> {

    #[inline]
    pub fn new(source: &'src [u8], backslash_quote: BackslashQuote) -> Self {
        let input = CharBuffer::new(source);
        Self { input, backslash_quote }
    }

    pub fn decode(&mut self) -> Result<String, ExtendedStringError> {

        // see `<xe>` and <xeu> rules in
        // [scan.l](https://github.com/postgres/postgres/blob/77761ee5dddc0518235a51c533893e81e5f375b9/src/backend/parser/scan.l#L275-L281)

        // b"''" => b'\''
        // b"\\'" => b'\''
        //   * backslash_quote in (SafeEncoding, Off) => Err (TO DO: check client encoding)
        //   * TO DO: check_string_escape_warning
        // [\\](b|f|n|r|t|v) => b'\b' | b'\f' | b'\n' | b'\r' | b'\t' | b'\v'
        // [\\][^0-7'bfnrtv\80-\ff] -> not an escape, just return the 2nd char and ignore the backslash
        // [\\][0-7]{1,3}
        // [\\]x[0-9A-Fa-f]{1,2}
        // unicode: [\\]u[0-9A-Fa-f]{4} => consume_unicode_char(4) (Ok(None) is an error here)
        // UNICODE: [\\]U[0-9A-Fa-f]{8} => consume_unicode_char(8) (Ok(None) is an error here)

        let mut out = Vec::with_capacity(self.input.source().len());

        while let Some(c) = self.input.consume_one() {

            if c == b'\'' {
                out.push(b'\'');
                // skip '' escape:
                self.input.consume_char(b'\'');
                continue
            }

            if c != b'\\' {
                out.push(c);
                continue
            }

            let c = match self.input.consume_one() {
                None => {
                    // this is possible: '\\'
                    // but this will complain about unterminated string: e'\\'
                    out.push(b'\\');
                    break
                }
                Some(c) => c,
            };

            match c {
                b'b' => out.push(b'\x08'), // '\b'
                b'f' => out.push(b'\x0c'), // '\f'
                b'n' => out.push(b'\n'),
                b'r' => out.push(b'\r'),
                b't' => out.push(b'\t'),
                b'v' => out.push(b'\x0b'), // '\v'
                b'\\' => {
                    // TODO: warn
                    out.push(b'\\')
                },
                b'\'' => { // b"\\'"
                    if self.forbid_backslash_quote() {
                        // TODO: check client encoding in the condition
                        return Err(NonstandardUseOfBackslashQuote)
                    }
                    out.push(b'\'')
                },
                b'0'..=b'7' => { // octal escape
                    let mut decoded = c - b'0';
                    if let Some(d) = self.input.consume_if(is_oct_digit) {
                        decoded = (decoded << 3) | (d - b'0');
                        if let Some(d) = self.input.consume_if(is_oct_digit) {
                            decoded = (decoded << 3) | (d - b'0');
                        }
                    }
                    out.push(decoded)
                },
                b'x' => { // hex escape
                    if let Some(d) = self.input.consume_if(is_hex_digit) {
                        let mut decoded = (d as char).to_digit(16).unwrap() as u8;
                        if let Some(d) = self.input.consume_if(is_hex_digit) {
                            let d = (d as char).to_digit(16).unwrap() as u8;
                            decoded = (decoded << 4) | d;
                        }
                        out.push(decoded)
                    }
                    else {
                        // not an escape
                        out.push(b'\\');
                        out.push(b'x')
                    }
                },
                b'u' | b'U' => {
                    let unicode_len = if c.is_ascii_lowercase() { 4 } else { 8 };

                    let c = match self.input.consume_unicode_char(unicode_len) {
                        Ok(UnicodeChar::Utf8(c)) => c,
                        Ok(SurrogateFirst(first)) => {
                            if !self.input.consume_char(b'\\') {
                                return Err(InvalidUnicodeSurrogatePair)
                            }

                            let unicode_len = if self.input.consume_char(b'u') {
                                4
                            }
                            else if self.input.consume_char(b'U') {
                                8
                            }
                            else {
                                return Err(InvalidUnicodeSurrogatePair)
                            };

                            match self.input.consume_unicode_char(unicode_len) {
                                Ok(SurrogateSecond(second)) => {
                                    wchar::decode_utf16(first, second)
                                        .ok_or(InvalidUnicodeSurrogatePair)?
                                },
                                _ => return Err(InvalidUnicodeSurrogatePair)
                            }
                        }
                        _ => return Err(InvalidUnicodeSurrogatePair),
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
                },
                _ => out.push(c),
            }
        }

        String::from_utf8(out)
            .map_err(|err| Utf8(err.utf8_error()))
    }

    fn forbid_backslash_quote(&self) -> bool {
        self.backslash_quote == BackslashQuote::Off || self.backslash_quote == SafeEncoding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_string() {
        let mut decoder = ExtendedStringDecoder::new(
            b"\\x64\\u0061\\164\\U00000061\\'\\\\''\\b\\f\\n\\r\\t\\v x\\y",
            BackslashQuote::On
        );
        assert_eq!(
            Ok("data'\\'\x08\x0c\n\r\t\x0b xy".to_string()),
            decoder.decode()
        )
    }
}
