pub(super) struct StringParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> StringParser<'p, 'src> {

    pub fn parse(&mut self) -> OptResult<String> {

        let Some((kind, loc)) = self.try_consume(false)? else { return Ok(None) };

        let slice = loc.slice(self.0.buffer.source());
        let slice = strip_delimiters(kind, slice);

        if kind == DollarString {
            // Not concatenable, and no escapes to deal with.

            return match std::str::from_utf8(slice) {
                Ok(string) => Ok(Some(string.to_owned())),
                Err(err) => Err(Some(err.into())),
            };
        }

        let mut string = slice.to_vec();

        let mut end_index = loc.range().end;
        while let Ok(Some((suffix_kind, suffix_loc))) = self.try_consume(true) {
            let suffix_slice = suffix_loc.slice(self.0.buffer.source());
            let suffix_slice = strip_delimiters(suffix_kind, suffix_slice);
            string.extend_from_slice(suffix_slice);
            end_index = suffix_loc.range().end;
        }

        let loc = Location::new(loc.range().start..end_index, loc.line(), loc.col());

        self.decode_string(kind, &string, loc)
    }

    fn try_consume(&mut self, only_concatenable: bool) -> OptResult<Located<StringKind>> {

        let loc = self.0.buffer.current_location();

        self.0.buffer.consume(|tok|
            tok.string_kind()
                .filter(|kind| {
                    !only_concatenable || match kind {
                        BasicString { concatenable }
                        | ExtendedString { concatenable } => *concatenable,
                        _ => false
                    }
                })
                .map(|kind| (kind, loc.clone()))
        )
    }

    fn decode_string(&mut self, kind: StringKind, slice: &[u8], loc: Location) -> OptResult<String> {

        let result = match kind {
            BasicString { .. } | NationalString => {
                BasicStringDecoder::new(slice, false)
                    .decode()
                    .map_err(Utf8Error::into)
            },
            ExtendedString { .. } => {
                let mut decoder = ExtendedStringDecoder::new(slice, self.0.config.backslash_quote());
                let ExtendedStringResult { result, warning } = decoder.decode();

                if let Some(warning) = warning {
                    let warning = ParserWarning::NonstandardEscape(warning);
                    self.0.warnings.push((warning, loc));
                }

                result.map_err(ParserErrorKind::ExtendedString)
            },
            UnicodeString => {

                let escape = self.0.uescape().map_err(Some)?;

                UnicodeStringDecoder::new(slice, false, escape)
                    .decode()
                    .map_err(ParserErrorKind::UnicodeString)
            }
            DollarString => unreachable!("`$` strings don't have any escapes"),
        };

        match result {
            Ok(result) => Ok(Some(result)),
            Err(err) => Err(Some(err)),
        }
    }
}

pub(super) fn strip_delimiters(kind: StringKind, slice: &[u8]) -> &[u8] {
    let range = match kind {
        DollarString => {
            let delim_len = slice.iter()
                .copied()
                .enumerate()
                .skip(1)
                .find(|(_, c)| *c == b'$')
                .map(|(i, _)| i + 1) // include the '$'
                .unwrap();

            let str_end = slice.len() - delim_len;
            delim_len..str_end
        }
        BasicString { .. } => 1..(slice.len() - 1),
        ExtendedString { .. } => {
            // `e'`, `n'`, or `'`
            let delim_len = if slice[0] == b'\'' { 1 } else { 2 };
            delim_len..(slice.len() - 1)
        }
        NationalString => 2..(slice.len() - 1),
        UnicodeString => 3..(slice.len() - 1),
    };

    &slice[range]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ParserConfig;
    use postgres_basics::guc::BackslashQuote;

    #[test]
    fn test_parse_basic_string() {
        let mut parser = new_parser(b"'a basic string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("a basic string".into())), result);
    }

    #[test]
    fn test_parse_basic_string_concatenable() {

        let mut parser = new_parser(
            b"'a basic string'\n\
            ' that concatenates'"
        );
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("a basic string that concatenates".into())), result);
    }

    #[test]
    fn test_dollar_string() {
        let mut parser = new_parser(b"$dollar$a $ string$dollar$");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("a $ string".into())), result);
    }

    #[test]
    fn test_unicode_string() {
        let mut parser = new_parser(br"u&'!0061n unicode string' UESCAPE '!'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("an unicode string".into())), result);
    }

    #[test]
    fn test_extended_string() {
        let mut parser = new_parser(br"e'\u0061n extended string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("an extended string".into())), result);
    }

    fn new_parser(source: &[u8]) -> Parser<'_> {
        let config = ParserConfig::new(true, BackslashQuote::SafeEncoding);
        Parser::new(source, config)
    }
}

use crate::lexer::{StringKind, StringKind::*};
use crate::parser::{token_buffer::TokenConsumer, OptResult, Parser, ParserErrorKind, ParserWarning};
use crate::string_decoders::*;
use postgres_basics::{Located, Location};
use std::str::Utf8Error;
