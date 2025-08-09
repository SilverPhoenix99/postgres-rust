#[derive(Debug)]
pub(super) struct BufferedLexer<'src> {
    pub lexer: Lexer<'src>,
    pub peek: Option<eof::Result<Located<RawTokenKind>>>,
    pub backslash_quote: BackslashQuote,
    /// All the warnings that have been collected while parsing.
    pub warnings: Vec<Located<Warning>>
}

impl BufferedLexer<'_> {

    pub fn next(&mut self) -> eof::Result<Located<RawTokenKind>> {
        match self.peek() {
            Ok(_) => self.peek.take().unwrap(),
            Err(err) => {
                // Don't consume to prevent moving forward.
                Err(err.clone())
            },
        }
    }

    fn peek(&mut self) -> &eof::Result<Located<RawTokenKind>> {

        self.peek.get_or_insert_with(|| {
            match self.lexer.next() {
                Some(Ok(tok)) => Ok(tok),
                Some(Err(lex_err)) => Err(lex_err.into()),
                None => {
                    let loc = self.lexer.current_location();
                    Err(Eof(loc))
                },
            }
        })
    }

    pub fn parse_identifier(&mut self, slice: &str, loc: Location, kind: IdentifierKind) -> eof::Result<Located<TokenValue>> {
        use IdentifierKind::{Basic, Unicode};

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
                        pg_elog::LocatedError::new(err, loc.clone())
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

        Ok(Located(ident, loc))
    }

    pub fn parse_bit_string(&mut self, slice: &str, loc: Location, kind: BitStringKind) -> eof::Result<Located<TokenValue>> {

        /*
            b'0101' ( SCONST )*
            x'01af' ( SCONST )*
        */

        // strip delimiters
        let slice = &slice[2..(slice.len() - 1)];

        let mut buffer = slice.to_owned();

        let mut end_position = loc.range().end;
        while let Some(Located(suffix, suffix_loc)) = self.next_concatenable_string() {
            buffer.push_str(suffix);
            end_position = suffix_loc.range().end;
        }

        let range = loc.range().start..end_position;
        let loc = Location::new(range, loc.line(), loc.col());

        let value = buffer.into_boxed_str();
        let value = TokenValue::BitString { value, kind };
        Ok(Located(value, loc))
    }

    pub fn parse_string(&mut self, slice: &str, loc: Location, kind: StringKind) -> eof::Result<Located<TokenValue>> {
        use StringKind::{Basic, Unicode};

        /*
              'String' ( SCONST )*
            | e'String' ( SCONST )*
            | n'String' ( SCONST )*
            | u&'String' ( SCONST )* ( UESCAPE ( SCONST )+ )?
        */

        let slice = strip_delimiters(kind, slice);
        let mut buffer = slice.to_owned();

        if kind == Dollar {
            // Not concatenable, and no escapes to deal with.
            let value = buffer.into_boxed_str();
            let value = TokenValue::String(value);
            return Ok(Located(value, loc));
        }

        let mut end_position = loc.range().end;
        while let Some(Located(suffix, suffix_loc)) = self.next_concatenable_string() {
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
                    self.warnings.push(Located(warning.into(), loc.clone()));
                }

                result.map_err(|err|
                    Located(err, loc.clone())
                )?
            }
            Unicode => {

                let escape = self.uescape()?;

                UnicodeStringDecoder::new(&buffer, false, escape)
                    .decode()
                    .map_err(|err|
                        Located(err, loc.clone())
                    )?
            }
            Dollar => unreachable!("`$` strings don't have any escapes"),
        };

        let value = TokenValue::String(string);
        Ok(Located(value, loc))
    }

    fn uescape(&mut self) -> pg_elog::LocatedResult<char> {
        use StringKind::Basic;

        /*
            ( UESCAPE ( SCONST )+ )?
        */

        let Ok(Located(Keyword(Uescape), _)) = self.peek() else { return Ok('\\') };
        let _ = self.next();

        let (kind, loc) = match self.peek() {

            Ok(Located(StringLiteral(kind @ (Basic { .. } | Extended { .. })), loc)) => (*kind, loc.clone()),

            // No match or Eof
            Ok(Located(_, loc))
            | Err(Eof(loc)) => {
                return Err(UescapeDelimiterMissing.into_located(loc.clone()))
            },

            Err(NotEof(err)) => return Err(err.clone()),
        };
        let _ = self.next();

        let slice = loc.slice(self.lexer.source());
        let slice = strip_delimiters(kind, slice);

        let mut buffer = slice.to_owned();

        let mut end_position = loc.range().end;
        while let Some(Located(suffix, suffix_loc)) = self.next_concatenable_string() {
            buffer.push_str(suffix);
            end_position = suffix_loc.range().end;
        }

        let range = loc.range().start..end_position;
        let loc = Location::new(range, loc.line(), loc.col());

        uescape_escape(&buffer).ok_or_else(||
            InvalidUescapeDelimiter.into_located(loc)
        )
    }

    fn next_concatenable_string(&mut self) -> Option<Located<&str>> {

        let (kind, loc) = {
            let Ok(Located(StringLiteral(kind), loc)) = self.peek() else { return None };
            if !kind.is_concatenable() {
                return None
            }
            (*kind, loc.clone())
        };
        let _ = self.next();

        let slice = loc.slice(self.lexer.source());
        let slice = strip_delimiters(kind, slice);
        Some(Located(slice, loc))
    }
}

use crate::eof;
use crate::eof::Error::Eof;
use crate::eof::Error::NotEof;
use crate::stream::string_decoders::BasicStringDecoder;
use crate::stream::string_decoders::ExtendedStringDecoder;
use crate::stream::string_decoders::ExtendedStringResult;
use crate::stream::string_decoders::UnicodeStringDecoder;
use crate::stream::strip_delimiters::strip_delimiters;
use crate::stream::token_value::TokenValue;
use crate::stream::uescape_escape::uescape_escape;
use pg_basics::guc::BackslashQuote;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_basics::Location;
use pg_basics::NAMEDATALEN;
use pg_elog::parser::Error::InvalidUescapeDelimiter;
use pg_elog::parser::Error::UescapeDelimiterMissing;
use pg_elog::parser::Warning;
use pg_lexer::BitStringKind;
use pg_lexer::IdentifierKind;
use pg_lexer::IdentifierKind::Quoted;
use pg_lexer::Keyword::Uescape;
use pg_lexer::Lexer;
use pg_lexer::RawTokenKind;
use pg_lexer::RawTokenKind::Keyword;
use pg_lexer::RawTokenKind::StringLiteral;
use pg_lexer::StringKind;
use pg_lexer::StringKind::Dollar;
use pg_lexer::StringKind::Extended;
