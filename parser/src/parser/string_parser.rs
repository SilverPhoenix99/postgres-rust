pub(super) struct StringParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> StringParser<'p, 'src> {

    pub fn parse(&mut self) -> ScanResult<String> {

        let (kind, slice, loc) = self.try_consume(false)?;

        let mut string = strip_delimiters(kind, slice).to_owned();

        if kind == Dollar {
            // Not concatenable, and no escapes to deal with.
            return Ok(string);
        }

        let mut end_index = loc.range().end;
        while let Some((suffix_kind, suffix_slice, suffix_loc)) = self.try_consume(true).optional()? {
            let suffix_slice = strip_delimiters(suffix_kind, suffix_slice);
            string.push_str(suffix_slice);
            end_index = suffix_loc.range().end;
        }

        let loc = Location::new(loc.range().start..end_index, loc.line(), loc.col());

        self.decode_string(kind, &string, loc)
    }

    fn try_consume(&mut self, only_concatenable: bool) -> ScanResult<(StringKind, &'src str, Location)> {

        self.0.buffer.consume_with_slice(|(tok, slice, loc)|
            tok.string()
                .filter(|kind| {
                    !only_concatenable || match kind {
                        Basic { concatenable }
                        | Extended { concatenable } => *concatenable,
                        _ => false
                    }
                })
                .map(|kind| (kind, slice, loc))
        )
    }

    fn decode_string(&mut self, kind: StringKind, slice: &str, loc: Location) -> ScanResult<String> {
        const FN_NAME: &str = "postgres_parser::parser::string_parser::StringParser::decode_string";

        let result = match kind {
            Basic { .. } => {
                let string = BasicStringDecoder::new(slice, false).decode();
                Ok(string)
            },
            Extended { .. } => {
                let mut decoder = ExtendedStringDecoder::new(slice, self.0.config.backslash_quote());
                let ExtendedStringResult { result, warning } = decoder.decode();

                if let Some(warning) = warning {
                    self.0.warnings.push((warning.into(), loc.clone()));
                }

                result.map_err(|err|
                    ParserError::new(ExtendedString(err), fn_info!(FN_NAME), loc)
                )
            },
            Unicode => {

                let escape = self.0.uescape()?;

                UnicodeStringDecoder::new(slice, false, escape)
                    .decode()
                    .map_err(|err|
                        ParserError::new(UnicodeString(err), fn_info!(FN_NAME), loc)
                    )
            }
            Dollar => unreachable!("`$` strings don't have any escapes"),
        };

        result.map_err(ScanErrorKind::from)
    }
}

pub(super) fn strip_delimiters(kind: StringKind, slice: &str) -> &str {
    let range = match kind {
        Dollar => {
            let delim_len = slice.chars()
                .enumerate()
                .skip(1)
                .find(|(_, c)| *c == '$')
                .map(|(i, _)| i + 1) // include the '$'
                .expect("$-string delimiter should exist");

            let str_end = slice.len() - delim_len;
            delim_len..str_end
        }
        Basic { .. } => 1..(slice.len() - 1),
        Extended { .. } => {
            // `e'`, `n'`, or `'`
            let delim_len = if slice.starts_with('\'') { 1 } else { 2 };
            delim_len..(slice.len() - 1)
        }
        Unicode => 3..(slice.len() - 1),
    };

    &slice[range]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Parser, ParserConfig};
    use postgres_basics::guc::BackslashQuote;

    #[test]
    fn test_parse_basic_string() {
        let mut parser = new_parser("'a basic string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok("a basic string".into()), result);
    }

    #[test]
    fn test_parse_basic_string_concatenable() {

        let mut parser = new_parser(
            "'a basic string'\n\
            ' that concatenates'"
        );
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok("a basic string that concatenates".into()), result);
    }

    #[test]
    fn test_dollar_string() {
        let mut parser = new_parser("$dollar$a $ string$dollar$");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok("a $ string".into()), result);
    }

    #[test]
    fn test_unicode_string() {
        let mut parser = new_parser(r"u&'!0061n unicode string' UESCAPE '!'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok("an unicode string".into()), result);
    }

    #[test]
    fn test_extended_string() {
        let mut parser = new_parser(r"e'\u0061n extended string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok("an extended string".into()), result);
    }

    fn new_parser(source: &str) -> Parser<'_> {
        let config = ParserConfig::new(true, BackslashQuote::SafeEncoding);
        Parser::new(source, config)
    }
}

use crate::{
    lexer::StringKind::{self, *},
    parser::{
        result::{Optional, ScanErrorKind, ScanResult},
        token_buffer::SlicedTokenConsumer,
        Parser,
        ParserError,
        ParserErrorKind::{ExtendedString, UnicodeString}
    },
    string_decoders::*
};
use postgres_basics::{fn_info, Location};
