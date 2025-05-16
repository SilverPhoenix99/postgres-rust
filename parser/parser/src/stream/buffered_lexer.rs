#[derive(Debug)]
pub(super) struct BufferedLexer<'src> {
    pub lexer: Lexer<'src>,
    pub peek: Option<EofResult<Located<RawTokenKind>>>,
    pub backslash_quote: BackslashQuote,
    /// All the warnings that have been collected while parsing.
    pub warnings: Vec<Located<ParserWarningKind>>
}

impl BufferedLexer<'_> {

    pub fn next(&mut self) -> EofResult<Located<RawTokenKind>> {
        match self.peek() {
            Ok(_) => self.peek.take().unwrap(),
            Err(err) => {
                // Don't consume to prevent moving forward.
                Err(err.clone())
            },
        }
    }

    fn peek(&mut self) -> &EofResult<Located<RawTokenKind>> {

        self.peek.get_or_insert_with(|| {
            match self.lexer.next() {
                Some(Ok(tok)) => Ok(tok),
                Some(Err(lex_err)) => Err(NotEof(lex_err.into())),
                None => {
                    let loc = self.lexer.current_location();
                    Err(Eof(loc))
                },
            }
        })
    }

    pub fn parse_identifier(&mut self, slice: &str, loc: Location, kind: IdentifierKind) -> EofResult<Located<TokenValue>> {
        use elog::parser::ParserError;
        use elog::parser::ParserErrorKind::UnicodeString;
        use postgres_basics::NAMEDATALEN;
        use postgres_parser_lexer::IdentifierKind::*;
        use crate::string_decoders::{BasicStringDecoder, UnicodeStringDecoder};

        /*
            An identifier is truncated to 64 chars.

              identifier
            | "Identifier"
            | u&"Identifier" ( UESCAPE ( SCONST )+ )?
        */

        let mut ident = match kind {
            Basic => slice.to_lowercase(),
            Quoted => {
                // Strip delimiters:
                let slice = &slice[1..slice.len() - 1];

                let ident = BasicStringDecoder::new(slice, true).decode();
                ident.into_string()
            }
            Unicode => {
                let escape = self.uescape()?;

                // Strip delimiters:
                let slice = &slice[3..slice.len() - 1];

                UnicodeStringDecoder::new(slice, true, escape)
                    .decode()
                    .map(str::into_string)
                    .map_err(|err|
                        ParserError::new(UnicodeString(err), loc.clone())
                    )?
            }
        };

        if ident.len() > NAMEDATALEN {
            let len: usize = ident.chars()
                .take(NAMEDATALEN)
                .map(char::len_utf8)
                .sum();
            if len < ident.len() {
                ident.truncate(len);
            }
        }

        let ident = ident.into_boxed_str();
        let ident = TokenValue::Identifier(ident);

        Ok((ident, loc))
    }

    pub fn parse_bit_string(&mut self, slice: &str, loc: Location, kind: BitStringKind) -> EofResult<Located<TokenValue>> {

        /*
            b'0101' ( SCONST )*
            x'01af' ( SCONST )*
        */

        // strip delimiters
        let slice = &slice[2..(slice.len() - 1)];

        let mut buffer = slice.to_owned();

        let mut end_position = loc.range().end;
        while let Some((suffix, suffix_loc)) = self.next_concatenable_string() {
            buffer.push_str(suffix);
            end_position = suffix_loc.range().end;
        }

        let range = loc.range().start..end_position;
        let loc = Location::new(range, loc.line(), loc.col());

        let value = buffer.into_boxed_str();
        let value = TokenValue::BitString { value, kind };
        Ok((value, loc))
    }

    pub fn parse_string(&mut self, slice: &str, loc: Location, kind: StringKind) -> EofResult<Located<TokenValue>> {
        use elog::parser::ParserError;
        use elog::parser::ParserErrorKind::{ExtendedString, UnicodeString};
        use postgres_parser_lexer::StringKind::*;
        use crate::stream::strip_delimiters;
        use crate::string_decoders::{BasicStringDecoder, ExtendedStringDecoder, ExtendedStringResult, UnicodeStringDecoder};

        /*
              'String' ( SCONST )*
            | e'String' ( SCONST )*
            | n'String' ( SCONST )*
            | u&'String' ( SCONST )* ( UESCAPE ( SCONST )+ )?
        */

        let slice = strip_delimiters::strip_delimiters(kind, slice);
        let mut buffer = slice.to_owned();

        if kind == Dollar {
            // Not concatenable, and no escapes to deal with.
            let value = buffer.into_boxed_str();
            let value = TokenValue::String(value);
            return Ok((value, loc));
        }

        let mut end_position = loc.range().end;
        while let Some((suffix, suffix_loc)) = self.next_concatenable_string() {
            buffer.push_str(suffix);
            end_position = suffix_loc.range().end;
        }

        let range = loc.range().start..end_position;
        let loc = Location::new(range, loc.line(), loc.col());

        let string = match kind {
            Basic { .. } => {
                BasicStringDecoder::new(&buffer, false).decode()
            }
            Extended { .. } => {

                let mut decoder = ExtendedStringDecoder::new(&buffer, self.backslash_quote);
                let ExtendedStringResult { result, warning } = decoder.decode();

                if let Some(warning) = warning {
                    self.warnings.push((warning.into(), loc.clone()));
                }

                result.map_err(|err|
                    ParserError::new(ExtendedString(err), loc.clone())
                )?
            }
            Unicode => {

                let escape = self.uescape()?;

                UnicodeStringDecoder::new(&buffer, false, escape)
                    .decode()
                    .map_err(|err|
                        ParserError::new(UnicodeString(err), loc.clone())
                    )?
            }
            Dollar => unreachable!("`$` strings don't have any escapes"),
        };

        let value = TokenValue::String(string);
        Ok((value, loc))
    }

    fn uescape(&mut self) -> ParseResult<char> {
        use elog::parser::ParserError;
        use elog::parser::ParserErrorKind::{InvalidUescapeDelimiter, UescapeDelimiterMissing};
        use postgres_parser_lexer::Keyword::Uescape;
        use postgres_parser_lexer::RawTokenKind::{Keyword as Kw, StringLiteral};
        use postgres_parser_lexer::StringKind::*;
        use crate::stream::strip_delimiters;
        use crate::stream::uescape_escape::uescape_escape;

        /*
            ( UESCAPE ( SCONST )+ )?
        */

        let Ok((Kw(Uescape), _)) = self.peek() else { return Ok('\\') };
        let _ = self.next();

        let (kind, loc) = match self.peek() {

            Ok((StringLiteral(kind @ (Basic { .. } | Extended { .. })), loc)) => (*kind, loc.clone()),

            // No match or Eof
            Ok((_, loc))
            | Err(Eof(loc)) => {
                return Err(
                    ParserError::new(UescapeDelimiterMissing, loc.clone())
                )
            },

            Err(NotEof(err)) => return Err(err.clone()),
        };
        let _ = self.next();

        let slice = loc.slice(self.lexer.source());
        let slice = strip_delimiters::strip_delimiters(kind, slice);

        let mut buffer = slice.to_owned();

        let mut end_position = loc.range().end;
        while let Some((suffix, suffix_loc)) = self.next_concatenable_string() {
            buffer.push_str(suffix);
            end_position = suffix_loc.range().end;
        }

        let range = loc.range().start..end_position;
        let loc = Location::new(range, loc.line(), loc.col());

        uescape_escape(&buffer).ok_or_else(||
            ParserError::new(InvalidUescapeDelimiter, loc)
        )
    }

    fn next_concatenable_string(&mut self) -> Option<Located<&str>> {

        let (kind, loc) = {
            let Ok((RawTokenKind::StringLiteral(kind), loc)) = self.peek() else { return None };
            if !kind.is_concatenable() {
                return None
            }
            (*kind, loc.clone())
        };
        let _ = self.next();

        let slice = loc.slice(self.lexer.source());
        let slice = strip_delimiters(kind, slice);
        Some((slice, loc))
    }
}

use crate::parser::ParseResult;
use crate::result::EofErrorKind::Eof;
use crate::result::EofErrorKind::NotEof;
use crate::result::EofResult;
use crate::stream::strip_delimiters::strip_delimiters;
use crate::stream::token_value::TokenValue;
use elog::parser::ParserWarningKind;
use postgres_basics::guc::BackslashQuote;
use postgres_basics::Located;
use postgres_basics::Location;
use postgres_parser_lexer::BitStringKind;
use postgres_parser_lexer::IdentifierKind;
use postgres_parser_lexer::Lexer;
use postgres_parser_lexer::RawTokenKind;
use postgres_parser_lexer::StringKind;
