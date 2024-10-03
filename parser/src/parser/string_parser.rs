pub(super) struct StringParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> StringParser<'p, 'src> {

    pub fn parse(&mut self) -> OptResult<String> {

        let Some((kind, loc)) = self.try_consume(false)? else { return Ok(None) };

        let slice = loc.slice(self.0.buffer.source());
        let mut string = strip_delimiters(kind, slice).to_owned();

        if kind == DollarString {
            // Not concatenable, and no escapes to deal with.
            return Ok(Some(string));
        }

        let mut end_index = loc.range().end;
        while let Ok(Some((suffix_kind, suffix_loc))) = self.try_consume(true) {
            let suffix_slice = suffix_loc.slice(self.0.buffer.source());
            let suffix_slice = strip_delimiters(suffix_kind, suffix_slice);
            string.push_str(suffix_slice);
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

    fn decode_string(&mut self, kind: StringKind, slice: &str, loc: Location) -> OptResult<String> {

        let result = match kind {
            BasicString { .. } | NationalString => {
                let string = BasicStringDecoder::new(slice, false).decode();
                Ok(string)
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

pub(super) fn strip_delimiters(kind: StringKind, slice: &str) -> &str {
    let range = match kind {
        DollarString => {
            let delim_len = slice.chars()
                .enumerate()
                .skip(1)
                .find(|(_, c)| *c == '$')
                .map(|(i, _)| i + 1) // include the '$'
                .unwrap();

            let str_end = slice.len() - delim_len;
            delim_len..str_end
        }
        BasicString { .. } => 1..(slice.len() - 1),
        ExtendedString { .. } => {
            // `e'`, `n'`, or `'`
            let delim_len = if slice.starts_with('\'') { 1 } else { 2 };
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
        let mut parser = new_parser("'a basic string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("a basic string".into())), result);
    }

    #[test]
    fn test_parse_basic_string_concatenable() {

        let mut parser = new_parser(
            "'a basic string'\n\
            ' that concatenates'"
        );
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("a basic string that concatenates".into())), result);
    }

    #[test]
    fn test_dollar_string() {
        let mut parser = new_parser("$dollar$a $ string$dollar$");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("a $ string".into())), result);
    }

    #[test]
    fn test_unicode_string() {
        let mut parser = new_parser(r"u&'!0061n unicode string' UESCAPE '!'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("an unicode string".into())), result);
    }

    #[test]
    fn test_extended_string() {
        let mut parser = new_parser(r"e'\u0061n extended string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some("an extended string".into())), result);
    }

    fn new_parser(source: &str) -> Parser<'_> {
        let config = ParserConfig::new(true, BackslashQuote::SafeEncoding);
        Parser::new(source, config)
    }
}

use crate::lexer::{StringKind, StringKind::*};
use crate::parser::{token_buffer::TokenConsumer, OptResult, Parser, ParserErrorKind, ParserWarning};
use crate::string_decoders::*;
use postgres_basics::{Located, Location};
