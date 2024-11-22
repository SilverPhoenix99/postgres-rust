
/// Aliases:
/// * `SCONST`
/// * `USCONST`
/// * `file_name`
pub(in crate::parser) fn string() -> StringCombi {
    StringCombi
}

pub(super) fn uescape() -> UescapeCombi {
    UescapeCombi
}

// (SCONST)* as long as they're concatenable.
// Internally used on productions that don't use UESCAPE.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::parser) struct StringCombi;

impl Combinator for StringCombi {
    type Output = Box<str>;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        let mut parser = InnerParser {
            uescape: ParseUescapeAfterwards,
            stream,
        };

        parser.parse()
            .map(|(string, _)| string)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum UescapeState {
    /// If it should parse UESCAPE SCONST
    ParseUescapeAfterwards,
    /// If the current string is on the right of the UESCAPE keyword,
    /// so it shouldn't parse it again
    IsUescape,
}

#[derive(Debug)]
pub(super) struct InnerParser<'stream, 'src> {
    uescape: UescapeState,
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

        match kind {
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
                    ParserError::new(ExtendedString(err), loc).into()
                )
            },
            Unicode => {

                // Unicode strings are not accepted as escape strings,
                // so if we're here, we can safely assume that
                // we're to the left of ( UESCAPE SCONST )?

                let escape = uescape().parse(self.stream)?;

                UnicodeStringDecoder::new(slice, false, escape)
                    .decode()
                    .map_err(|err|
                        ParserError::new(UnicodeString(err), loc).into()
                    )
            }
            Dollar => unreachable!("`$` strings don't have any escapes"),
        }
    }
}

fn accept(uescape: UescapeState, kind: StringKind, is_first: bool) -> bool {

    if is_first {
        // It's the 1st piece of the string, and:
        return
            // * we're *Not* to the right of a UESCAPE;
            uescape == ParseUescapeAfterwards
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct UescapeCombi;

impl Combinator for UescapeCombi {
    type Output = char;

    fn parse(&self, stream: &mut TokenStream<'_>) -> ScanResult<Self::Output> {

        if keyword(Uescape).parse(stream).optional()?.is_none() {
            return Ok('\\')
        }

        let mut parser = InnerParser {
            uescape: IsUescape,
            stream
        };

        let (escape, loc) = match parser.parse() {
            Ok(ok) => ok,
            Err(ScanErr(err)) => return Err(err.into()),
            Err(NoMatch(loc) | Eof(loc)) => {
                return Err(
                    ParserError::new(UescapeDelimiterMissing, loc).into()
                )
            },
        };

        let escape = uescape_escape(&escape);
        escape.ok_or_else(||
            ParserError::new(InvalidUescapeDelimiter, loc).into()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::DEFAULT_CONFIG;
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
        let parser = string();
        let actual = parser.parse(&mut stream);
        assert_eq!(expected, actual.unwrap().as_ref())
    }
}

use crate::lexer::Keyword::Uescape;
use crate::lexer::RawTokenKind::StringLiteral;
use crate::lexer::StringKind;
use crate::lexer::StringKind::Basic;
use crate::lexer::StringKind::Dollar;
use crate::lexer::StringKind::Extended;
use crate::lexer::StringKind::Unicode;
use crate::parser::combinators::keyword;
use crate::parser::combinators::uescape_escape::uescape_escape;
use crate::parser::combinators::Combinator;
use crate::parser::result::Optional;
use crate::parser::result::ScanErrorKind::Eof;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanErrorKind::ScanErr;
use crate::parser::result::ScanResult;
use crate::parser::token_stream::SlicedTokenConsumer;
use crate::parser::token_stream::TokenStream;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::ExtendedString;
use crate::parser::ParserErrorKind::InvalidUescapeDelimiter;
use crate::parser::ParserErrorKind::UescapeDelimiterMissing;
use crate::parser::ParserErrorKind::UnicodeString;
use crate::string_decoders::BasicStringDecoder;
use crate::string_decoders::ExtendedStringDecoder;
use crate::string_decoders::ExtendedStringResult;
use crate::string_decoders::UnicodeStringDecoder;
use postgres_basics::Location;
use UescapeState::{IsUescape, ParseUescapeAfterwards};
