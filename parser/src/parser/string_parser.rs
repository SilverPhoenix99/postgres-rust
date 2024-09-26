pub struct StringParserResult {
    pub result: OptResult<AstNode>,
    pub warning: Option<Located<ExtendedStringWarning>>
}

impl StringParserResult {

    #[inline(always)]
    fn string_warn(string: String, warning: Located<ExtendedStringWarning>) -> Self {
        let mut res = Self::string(string);
        res.warning = Some(warning);
        res
    }

    #[inline(always)]
    fn string(string: String) -> Self {
        Self {
            result: Ok(Some(StringLiteral(string))),
            warning: None
        }
    }

    #[inline(always)]
    fn err_warn(err: ParserErrorKind, warning: Located<ExtendedStringWarning>) -> Self {
        let mut res = Self::err(err);
        res.warning = Some(warning);
        res
    }

    #[inline(always)]
    fn err(err: ParserErrorKind) -> Self {
        Self {
            result: Err(Some(err)),
            warning: None
        }
    }

    #[inline(always)]
    fn eof() -> Self {
        Self {
            result: Err(None),
            warning: None
        }
    }

    #[inline(always)]
    fn no_match() -> Self {
        Self {
            result: Ok(None),
            warning: None
        }
    }
}

pub(super) struct StringParser<'p, 'src>(
    pub &'p mut Parser<'src>,
);

impl<'p, 'src> StringParser<'p, 'src> {

    pub fn parse(&mut self) -> StringParserResult {

        let (kind, loc) = match self.try_consume(false) {
            Ok(Some(tok)) => tok,
            Ok(None) => return StringParserResult::no_match(),
            Err(None) => return StringParserResult::eof(),
            Err(Some(err)) => return StringParserResult::err(err),
        };

        let slice = loc.slice(self.0.buffer.source());
        let slice = strip_delimiters(kind, slice);

        if kind == DollarString {
            // Not concatenable, and no escapes to deal with.

            return match std::str::from_utf8(slice) {
                Ok(string) => StringParserResult::string(string.into()),
                Err(err) => StringParserResult::err(err.into()),
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

    fn decode_string(&mut self, kind: StringKind, slice: &[u8], loc: Location) -> StringParserResult {

        let (result, warning) = match kind {
            BasicString { .. } | NationalString => {
                let result = BasicStringDecoder::new(slice, false)
                    .decode()
                    .map(StringLiteral)
                    .map_err(Utf8Error::into);
                (result, None)
            },
            ExtendedString { .. } => {
                let mut decoder = ExtendedStringDecoder::new(slice, self.0.config.backslash_quote());
                let ExtendedStringResult { result, warning } = decoder.decode();

                let result = match result {
                    Ok(string) => Ok(StringLiteral(string)),
                    Err(err) => Err(ParserErrorKind::ExtendedString(err)),
                };

                (result, warning)
            },
            UnicodeString => {

                let escape = match self.0.uescape() {
                    Ok(escape) => escape,
                    Err(err) => return StringParserResult::err(err),
                };

                let result = UnicodeStringDecoder::new(slice, false, escape)
                    .decode()
                    .map(StringLiteral)
                    .map_err(ParserErrorKind::UnicodeString);

                (result, None)
            }
            BinaryString | HexString => {
                let result = BitStringDecoder::new(slice, kind == HexString)
                    .decode()
                    .map(BitStringLiteral)
                    .map_err(BitStringError::into);

                (result, None)
            }
            DollarString => unreachable!("`$` strings don't have any escapes"),
        };

        StringParserResult {
            result: result.map(Some).map_err(Some),
            warning: warning.map(|w| (w, loc)),
        }
    }
}

fn strip_delimiters(kind: StringKind, slice: &[u8]) -> &[u8] {
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
        BinaryString | HexString | NationalString => 2..(slice.len() - 1),
        UnicodeString => 3..(slice.len() - 1),
    };

    &slice[range]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ParseMode, ParserConfig};
    use postgres_basics::guc::BackslashQuote;

    #[test]
    fn test_parse_basic_string() {
        let mut parser = new_parser(b"'a basic string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some(StringLiteral("a basic string".into()))), result.result);
    }

    #[test]
    fn test_parse_basic_string_concatenable() {

        let mut parser = new_parser(
            b"'a basic string'\n\
            ' that concatenates'"
        );
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some(StringLiteral("a basic string that concatenates".into()))), result.result);
    }

    #[test]
    fn test_dollar_string() {
        let mut parser = new_parser(b"$dollar$a $ string$dollar$");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some(StringLiteral("a $ string".into()))), result.result);
    }

    #[test]
    fn test_unicode_string() {
        let mut parser = new_parser(br"u&'!0061n unicode string' UESCAPE '!'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some(StringLiteral("an unicode string".into()))), result.result);
    }

    #[test]
    fn test_extended_string() {
        let mut parser = new_parser(br"e'\u0061n extended string'");
        let mut string_parser = StringParser(&mut parser);

        let result = string_parser.parse();
        assert_eq!(Ok(Some(StringLiteral("an extended string".into()))), result.result);
    }

    fn new_parser(source: &[u8]) -> Parser<'_> {

        let config = ParserConfig::new(
            true,
            BackslashQuote::SafeEncoding,
            ParseMode::Default
        );

        Parser::new(source, config)
    }
}

use crate::lexer::{StringKind, StringKind::*};
use crate::parser::token_buffer::TokenConsumer;
use crate::parser::{AstNode, AstNode::*, OptResult, Parser, ParserErrorKind};
use crate::string_decoders::*;
use postgres_basics::{Located, Location};
use std::str::Utf8Error;
