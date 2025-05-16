pub(in crate::stream) struct ExtendedStringDecoder<'src> {
    input: CharBuffer<'src>,
    backslash_quote: BackslashQuote,
}

pub(in crate::stream) struct ExtendedStringResult {
    pub result: Result<Box<str>, ExtendedStringError>,
    pub warning: Option<ExtendedStringWarning>,
}

impl<'src> ExtendedStringDecoder<'src> {

    #[inline]
    pub fn new(source: &'src str, backslash_quote: BackslashQuote) -> Self {
        let input = CharBuffer::new(source);
        Self { input, backslash_quote }
    }

    pub fn decode(&mut self) -> ExtendedStringResult {
        // see `<xe>` and <xeu> rules in
        // [scan.l](https://github.com/postgres/postgres/blob/77761ee5dddc0518235a51c533893e81e5f375b9/src/backend/parser/scan.l#L275-L281)

        // "''" => '\''
        // "\\'" => '\''
        //   * backslash_quote in (SafeEncoding, Off) => Err (TO DO: check client encoding)
        //   * TO DO: check_string_escape_warning
        // [\\](b|f|n|r|t|v) => '\b' | '\f' | '\n' | '\r' | '\t' | '\v'
        // [\\][^0-7'bfnrtv\80-\ff] -> not an escape, just return the 2nd char and ignore the backslash
        // [\\][0-7]{1,3}
        // [\\]x[0-9A-Fa-f]{1,2}
        // unicode: [\\]u[0-9A-Fa-f]{4} => consume_unicode_char(4) (Ok(None) is an error here)
        // UNICODE: [\\]U[0-9A-Fa-f]{8} => consume_unicode_char(8) (Ok(None) is an error here)

        let mut out = Vec::<u8>::with_capacity(self.input.source().len());
        let mut warning: Option<ExtendedStringWarning> = None;

        while let Some(c) = self.input.consume_one() {

            if c == '\'' {
                out.push(b'\'');
                // skip '' escape:
                self.input.consume_char('\'');
                continue
            }

            if c != '\\' {
                out.push(c as u8);
                continue
            }

            let Some(c) = self.input.consume_one() else {
                // this is possible: '\\'
                // but this will complain about unterminated string: e'\\'
                out.push(b'\\');
                break
            };

            match c {
                'b' => out.push(b'\x08'), // '\b'
                'f' => out.push(b'\x0c'), // '\f'
                'n' => out.push(b'\n'),
                'r' => out.push(b'\r'),
                't' => out.push(b'\t'),
                'v' => out.push(b'\x0b'), // '\v'
                '\\' => {
                    warning.get_or_insert(NonstandardBackslashEscape);
                    out.push(b'\\')
                },
                '\'' => { // "\\'"
                    if self.forbid_backslash_quote() {
                        // TODO: check client encoding in the condition
                        return ExtendedStringResult {
                            result: Err(NonstandardUseOfBackslashQuote),
                            warning
                        }
                    }
                    warning.get_or_insert(NonstandardQuoteEscape);
                    out.push(b'\'')
                },
                '0'..='7' => { // octal escape

                    // C-PG doc: It is your responsibility that the byte sequences you create,
                    //           especially when using the octal or hexadecimal escapes,
                    //           compose valid characters in the server character set encoding.
                    // Any octal with 3 digits can go above 0xff, so we'll reproduce here discarding the high bit,
                    // like C-PG implicitly does by casting to `unsigned char`.

                    let mut decoded = c.to_digit(8).expect("should be octal char");
                    if let Some(c) = self.input.consume_if(is_oct_digit) {
                        decoded = (decoded << 3) | c.to_digit(8).expect("should be octal char");
                        if let Some(c) = self.input.consume_if(is_oct_digit) {
                            decoded = (decoded << 3) | c.to_digit(8).expect("should be octal char");
                        }
                    }

                    // TODO: check 0x00

                    // This is where we strip the high bit (0x1__) with the cast.
                    out.push(decoded as u8)
                },
                'x' => { // hex escape
                    if let Some(c) = self.input.consume_if(is_hex_digit) {
                        let mut decoded = c.to_digit(16).expect("should be hex char");
                        if let Some(c) = self.input.consume_if(is_hex_digit) {
                            let c = c.to_digit(16).expect("should be hex char");
                            decoded = (decoded << 4) | c;
                        }
                        // SAFETY: The maximum value is 0xff.
                        out.push(decoded as u8)
                    }
                    else {
                        // not an escape
                        out.push(b'\\');
                        out.push(b'x')
                    }
                },
                'u' | 'U' => {

                    warning.get_or_insert(NonstandardEscape);

                    let unicode_len = if c.is_ascii_lowercase() { 4 } else { 8 };

                    let c = match self.consume_unicode(unicode_len) {
                        Ok(c) => c,
                        Err(err) => return ExtendedStringResult {
                            result: Err(err),
                            warning
                        }
                    };

                    push_char(&mut out, c)
                },
                _ => push_char(&mut out, c),
            }
        }

        let result = String::from_utf8(out)
            .map(String::into_boxed_str)
            .map_err(|err| {
                // pg_verifymbstr -> pg_verify_mbstr -> report_invalid_encoding
                // see [report_invalid_encoding](https://github.com/postgres/postgres/blob/d5622acb32b3c11a27b323138fbee9c715742b38/src/backend/utils/mb/mbutils.c#L1698-L1721)
                Utf8(err.utf8_error())
            });

        ExtendedStringResult { result, warning }
    }

    fn consume_unicode(&mut self, unicode_len: u32) -> Result<char, ExtendedStringError> {

        let start_index = self.input.current_index() - 2; // include `\u`

        let first = match self.input.consume_unicode_char(unicode_len) {
            Ok(UnicodeChar::Utf8(c)) => return Ok(c),
            Ok(SurrogateFirst(first)) => Ok(first),
            Ok(SurrogateSecond(_)) => Err(InvalidUnicodeSurrogatePair(start_index)),
            Err(LenTooShort) => Err(InvalidUnicodeEscape(start_index)),
            Err(UnicodeCharError::InvalidUnicodeValue) => Err(InvalidUnicodeValue(start_index)),
        }?;

        let start_index = self.input.current_index();
        let invalid_pair = InvalidUnicodeSurrogatePair(start_index);
        if !self.input.consume_char('\\') {
            return Err(invalid_pair)
        }

        let unicode_len = if self.input.consume_char('u') {
            4
        }
        else if self.input.consume_char('U') {
            8
        }
        else {
            return Err(invalid_pair)
        };

        let Ok(SurrogateSecond(second)) = self.input.consume_unicode_char(unicode_len) else {
            return Err(invalid_pair)
        };

        wchar::decode_utf16(first, second).ok_or(invalid_pair)
    }

    fn forbid_backslash_quote(&self) -> bool {
        self.backslash_quote == BackslashQuote::Off || self.backslash_quote == SafeEncoding
    }
}

fn push_char(buffer: &mut Vec<u8>, c: char) {

    let len = c.len_utf8();
    if len == 1 {
        // fast path
        buffer.push(c as u8);
        return
    }

    // Avoid allocating a string, by encoding the char directly,
    // and pushing the raw bytes directly into the output buffer.
    let mut buff = [0; 4];
    c.encode_utf8(&mut buff);
    buffer.extend_from_slice(&buff[..len]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_string() {
        let mut decoder = ExtendedStringDecoder::new(
            r"\x64\u0061\164\U00000061\'\\''\b\f\n\r\t\v x\y",
            BackslashQuote::On
        );
        assert_eq!("data'\\'\x08\x0c\n\r\t\x0b xy", decoder.decode().result.unwrap().as_ref())
    }
}

use elog::extended_string::ExtendedStringError;
use elog::extended_string::ExtendedStringError::*;
use elog::extended_string::ExtendedStringWarning;
use elog::extended_string::ExtendedStringWarning::*;
use postgres_basics::ascii::is_hex_digit;
use postgres_basics::ascii::is_oct_digit;
use postgres_basics::guc::BackslashQuote;
use postgres_basics::guc::BackslashQuote::SafeEncoding;
use postgres_basics::wchar;
use postgres_basics::CharBuffer;
use postgres_basics::UnicodeChar;
use postgres_basics::UnicodeChar::SurrogateFirst;
use postgres_basics::UnicodeChar::SurrogateSecond;
use postgres_basics::UnicodeCharError;
use postgres_basics::UnicodeCharError::LenTooShort;
