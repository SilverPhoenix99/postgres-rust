
/// Aliases:
/// * `SCONST`
/// * `USCONST`
/// * `file_name`
pub(in crate::parser) fn string(caller: &'static FnInfo) -> StringCombi {
    StringCombi {
        uescape: UescapeState::ParseUescapeAfterwards,
        caller
    }
}

pub(super) fn uescape(caller: &'static FnInfo) -> UescapeCombi {
    UescapeCombi { caller }
}

// (SCONST)* as long as they're concatenable.
// Internally used on productions that don't use UESCAPE.
pub(in crate::parser) struct StringCombi {
    uescape: UescapeState,
    caller: &'static FnInfo,
}

impl ParserFunc for StringCombi {
    type Output = Box<str>;
    type Error = ScanErrorKind;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let mut parser = InnerParser {
            uescape: self.uescape,
            caller: self.caller,
            stream,
        };

        parser.parse()
            .map(|(string, _)| string)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum UescapeState {
    /// If it should parse UESCAPE SCONST
    ParseUescapeAfterwards,
    /// If the current string is on the right of the UESCAPE keyword,
    /// so it shouldn't parse it again
    IsUescape,
}

pub(super) struct InnerParser<'stream, 'src> {
    uescape: UescapeState,
    caller: &'static FnInfo,
    stream: &'stream mut TokenStream<'src>,
}

impl InnerParser<'_, '_> {

    fn parse(&mut self) -> ScanResult<(Box<str>, Location)> {

        let (kind, slice, loc) = self.try_consume(true)?;
        let mut string = slice.to_owned();

        if kind == Dollar {
            // Not concatenable, and no escapes to deal with.
            return Ok((string.into_boxed_str(), loc));
        }

        let mut end_index = loc.range().end;
        while let Some((_, suffix_slice, suffix_loc)) = self.try_consume(false).optional()? {
            string.push_str(suffix_slice);
            end_index = suffix_loc.range().end;
        }

        let range = loc.range().start..end_index;
        let loc = Location::new(range, loc.line(), loc.col());

        let string = self.decode_string(kind, &string, loc.clone())?;

        Ok((string, loc))
    }

    fn try_consume(&mut self, is_first: bool) -> ScanResult<(StringKind, &str, Location)> {
        let uescape = self.uescape;
        self.stream.consume_with_slice(|(tok, slice, loc)| {
            let StringLiteral(kind) = tok else { return None };
            if !accept(uescape, kind, is_first) { return None }
            let slice = strip_delimiters(kind, slice);
            Some((kind, slice, loc))
        })
    }

    fn decode_string(&mut self, kind: StringKind, slice: &str, loc: Location) -> ScanResult<Box<str>> {

        let result = match kind {
            Basic { .. } => {
                let string = BasicStringDecoder::new(slice, false).decode();
                Ok(string)
            },
            Extended { .. } => {
                let mut decoder = ExtendedStringDecoder::new(slice, self.stream.backslash_quote());
                let ExtendedStringResult { result, warning } = decoder.decode();

                if let Some(warning) = warning {
                    self.stream.warnings().push((warning.into(), loc.clone()));
                }

                result.map_err(|err|
                    ParserError::new(ExtendedString(err), self.caller, loc)
                )
            },
            Unicode => {

                // Unicode strings are not accepted as escape strings,
                // so if we're here, we can safely assume that
                // we're to the left of ( UESCAPE SCONST )?

                let escape = uescape(self.caller).parse(self.stream)?;

                UnicodeStringDecoder::new(slice, false, escape)
                    .decode()
                    .map_err(|err|
                        ParserError::new(UnicodeString(err), self.caller, loc)
                    )
            }
            Dollar => unreachable!("`$` strings don't have any escapes"),
        };

        result.map_err(ScanErrorKind::from)
    }
}

fn accept(uescape: UescapeState, kind: StringKind, is_first: bool) -> bool {

    if is_first {
        // It's the 1st piece of the string, and:
        return
            // * we're *Not* to the right of a UESCAPE;
            uescape == UescapeState::ParseUescapeAfterwards
            // * or, we are, and it's a "simple" string.
            || matches!(kind, Basic { .. } | Extended { .. })
    }

    // If it's a follow-up piece, then only accept it if it's concatenable.
    kind.is_concatenable()
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

pub(super) struct UescapeCombi {
    caller: &'static FnInfo,
}

impl ParserFunc for UescapeCombi {
    type Output = char;
    type Error = ParserError;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ParseResult<Self::Output> {

        if keyword(Uescape).parse(stream).optional()?.is_none() {
            return Ok('\\')
        }

        let mut parser = InnerParser {
            uescape: UescapeState::IsUescape,
            caller: self.caller,
            stream
        };

        let (escape, loc) = match parser.parse() {
            Ok(ok) => ok,
            Err(ScanErr(err)) => return Err(err),
            Err(NoMatch(loc) | Eof(loc)) => {
                return Err(
                    ParserError::new(UescapeDelimiterMissing, self.caller, loc)
                )
            },
        };

        let escape = uescape_escape(&escape);
        escape.ok_or_else(||
            ParserError::new(InvalidUescapeDelimiter, self.caller, loc)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
    use postgres_basics::fn_info;
    use test_case::test_case;

    #[test_case("$dollar$a $ string$dollar$", "a $ string")]
    #[test_case("'basic string'", "basic string")]
    #[test_case("'basic ''string'''\n' concatenation'", "basic 'string' concatenation")]
    #[test_case(r"e'\u0061n extended string'", "an extended string")]
    #[test_case("e'extended string'\n' concatenation'", "extended string concatenation")]
    #[test_case(r"u&'\0061n unicode string'", "an unicode string")]
    #[test_case("u&'!0061n escaped unicode string!0021' UESCAPE '!'", "an escaped unicode string!")]
    #[test_case("u&'unicode string'\n' concatenation'", "unicode string concatenation")]
    #[test_case("u&'*002a extended unicode *002a' UESCAPE e'*'", "* extended unicode *")]
    #[test_case("u&'unicode esc!0061pe concatenation' UESCAPE ''\n''\n'!'", "unicode escape concatenation")]
    fn test_string(source: &str, expected: &str) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let parser = string(fn_info!());
        let actual = parser.parse(&mut stream);
        assert_eq!(expected, actual.unwrap().as_ref())
    }
}

use crate::parser::ParseResult;
use crate::{
    lexer::{
        Keyword::Uescape,
        RawTokenKind::StringLiteral,
        StringKind::{self, Basic, Dollar, Extended, Unicode}
    },
    parser::{
        combinators::{keyword, ParserFunc},
        result::{
            Optional,
            ScanErrorKind::{self, Eof, NoMatch, ScanErr},
            ScanResult
        },
        token_stream::{SlicedTokenConsumer, TokenStream},
        uescape_escape::uescape_escape,
        ParserError,
        ParserErrorKind::{ExtendedString, InvalidUescapeDelimiter, UescapeDelimiterMissing, UnicodeString}
    },
    string_decoders::{
        BasicStringDecoder,
        ExtendedStringDecoder,
        ExtendedStringResult,
        UnicodeStringDecoder
    }
};
use postgres_basics::{FnInfo, Location};
