#[derive(Debug)]
pub(super) struct TokenStream<'src> {
    lexer: BufferedLexer<'src>,
    buf: VecDeque<EofResult<Located<TokenValue>>>
}

impl<'src> TokenStream<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        let lexer = Lexer::new(source, config.standard_conforming_strings());
        Self::with_lexer(lexer, config.backslash_quote())
    }

    pub fn with_lexer(lexer: Lexer<'src>, backslash_quote: BackslashQuote) -> Self {
        Self {
            lexer: BufferedLexer {
                lexer,
                peek: None,
                backslash_quote,
                warnings: Vec::new()
            },
            buf: VecDeque::with_capacity(2),
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &'src str {
        self.lexer.lexer.source()
    }

    pub fn warnings(&mut self) -> &mut Vec<Located<ParserWarningKind>> {
        &mut self.lexer.warnings
    }

    #[inline(always)]
    pub fn eof(&mut self) -> bool {
        matches!(self.peek_mut(), Err(Eof(_)))
    }

    /// Returns the location of the current token,
    /// or an empty-length location if in the Eof state.
    #[inline(always)]
    pub fn current_location(&mut self) -> Location {
        match self.peek_mut() {
            Ok((_, loc)) | Err(Eof(loc)) => loc.clone(),
            Err(NotEof(err)) => err.location().clone(),
        }
    }

    pub fn slice(&mut self) -> Option<&'src str> {

        let source = self.source();

        let Ok((_, loc)) = self.peek_mut() else {
            return None
        };

        let slice = loc.slice(source);
        Some(slice)
    }

    #[inline(always)]
    pub fn next(&mut self) {
        self.buf.pop_front();
    }

    #[inline(always)]
    pub fn peek(&mut self) -> EofResult<&TokenValue> {
        match self.peek_mut() {
            Ok((tok, _)) => Ok(tok),
            Err(err) => Err(err.clone()),
        }
    }

    fn peek_mut(&mut self) -> &mut EofResult<Located<TokenValue>> {

        self.fill_buf();

        // SAFETY: `fill_buf()` always adds 2 elements to `self.buf`,
        //         even when the lexer is done
        self.buf.front_mut().unwrap()
    }

    pub fn peek2(&mut self) -> (EofResult<&TokenValue>, EofResult<&TokenValue>) {

        self.fill_buf();

        // SAFETY: `fill_buf()` always adds 2 elements to `self.buf`,
        //         even when the lexer is in Eof
        let first = self.buf.front()
            .expect("first element missing: `fill_buf()` should have filled 2 elements into `self.buf`");
        let second = self.buf.get(1)
            .expect("second element missing: `fill_buf()` should have filled 2 elements into `self.buf`");

        let first = match first {
            Ok((tok, _)) => Ok(tok),
            Err(err) => Err(err.clone()),
        };

        let second = match second {
            Ok((tok, _)) => Ok(tok),
            Err(err) => Err(err.clone()),
        };

        (first, second)
    }

    fn fill_buf(&mut self) {
        while self.buf.len() < 2 {
            let result = self.lex_next();
            self.buf.push_back(result);
        }
    }

    fn lex_next(&mut self) -> EofResult<Located<TokenValue>> {
        use RawTokenKind::*;

        let (tok, loc) = self.lexer.next()?;
        let slice = loc.slice(self.source());

        match tok {
            Operator(op) => Ok((TokenValue::Operator(op), loc)),
            Keyword(kw) => Ok((TokenValue::Keyword(kw), loc)),
            Param { index } => Ok((TokenValue::Param { index }, loc)),
            UserDefinedOperator => {
                let value = TokenValue::UserDefinedOperator(slice.into());
                Ok((value, loc))
            },
            NumberLiteral(radix) => {
                let value = parse_number(slice, radix);
                let value = TokenValue::UnsignedNumber(value);
                Ok((value, loc))
            },
            BitStringLiteral(kind) => self.lexer.parse_bit_string(slice, loc, kind),
            Identifier(kind) => self.lexer.parse_identifier(slice, loc, kind),
            StringLiteral(kind) => self.lexer.parse_string(slice, loc, kind),
        }
    }
}

fn parse_number(value: &str, radix: NumberRadix) -> UnsignedNumber {

    let value = value.replace("_", "");

    if let Ok(int) = i32::from_str_radix(&value, radix as u32) {
        // SAFETY: `0 <= int <= i32::MAX`
        IntegerConst(int.into())
    }
    else {
        NumericConst {
            radix,
            value: value.into_boxed_str()
        }
    }
}

pub(super) trait TokenConsumer<TOut, FRes> {
    fn consume<F>(&mut self, f: F) -> ScanResult<TOut>
    where
        F: Fn(&mut TokenValue) -> FRes;
}

/// Consumers are not allowed to return `Err(Eof)`,
/// which is an internal error that's only returned by the `TokenBuffer` directly.
pub(super) type ConsumerResult<T> = ParseResult<Option<T>>;

impl<TOut> TokenConsumer<TOut, ConsumerResult<TOut>> for TokenStream<'_> {
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(&mut TokenValue) -> ConsumerResult<TOut>
    {
        let (tok, loc) = match self.peek_mut() {
            Ok(ok) => ok,
            Err(err) => return Err(err.clone().into()),
        };

        match mapper(tok) {

            // Some parse error was returned
            Err(err) => Err(ScanErr(err)),

            // No match
            Ok(None) => Err(NoMatch(loc.clone())),

            // The mapper matched the token
            Ok(Some(ok)) => {
                // Consume it from the Lexer.
                self.next();
                Ok(ok)
            },
        }
    }
}

impl<TOut> TokenConsumer<TOut, Option<TOut>> for TokenStream<'_> {
    #[inline(always)]
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(&mut TokenValue) -> Option<TOut>
    {
        self.consume(|tok| Ok(mapper(tok)))
    }
}

impl TokenConsumer<TokenValue, bool> for TokenStream<'_> {
    #[inline(always)]
    fn consume<P>(&mut self, pred: P) -> ScanResult<TokenValue>
    where
        P: Fn(&mut TokenValue) -> bool
    {
        let (tok, loc) = match self.peek_mut() {
            Ok(ok) => ok,
            Err(err) => return Err(err.clone().into()),
        };

        if !pred(tok) {
            return Err(NoMatch(loc.clone()))
        }

        // SAFETY: `tok` already matched
        let (tok, _) = self.buf.pop_front().unwrap()?;
        Ok(tok)
    }
}

#[derive(Debug)]
struct BufferedLexer<'src> {
    lexer: Lexer<'src>,
    peek: Option<EofResult<Located<RawTokenKind>>>,
    backslash_quote: BackslashQuote,
    /// All the warnings that have been collected while parsing.
    warnings: Vec<Located<ParserWarningKind>>
}

impl BufferedLexer<'_> {

    fn next(&mut self) -> EofResult<Located<RawTokenKind>> {
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

    fn parse_identifier(&mut self, slice: &str, loc: Location, kind: IdentifierKind) -> EofResult<Located<TokenValue>> {
        use IdentifierKind::*;

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

    fn parse_bit_string(&mut self, slice: &str, loc: Location, kind: BitStringKind) -> EofResult<Located<TokenValue>> {

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

    fn parse_string(&mut self, slice: &str, loc: Location, kind: StringKind) -> EofResult<Located<TokenValue>> {
        use StringKind::*;

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
        use RawTokenKind::{Keyword as Kw, StringLiteral};
        use StringKind::*;

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
        let slice = strip_delimiters(kind, slice);

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

fn strip_delimiters(kind: StringKind, slice: &str) -> &str {
    use StringKind::*;

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
    use crate::parser::tests::DEFAULT_CONFIG;
    use elog::parser::ParserErrorKind::Syntax;
    use TokenValue::Identifier;

    #[test]
    fn test_eof() {
        let mut buffer =  TokenStream::new("", DEFAULT_CONFIG);

        assert!(buffer.eof())
    }

    #[test]
    fn test_next_and_peek_and_current_location() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        assert_matches!(buffer.peek(), Ok(_));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());

        buffer.next();

        assert_matches!(buffer.peek(), Ok(_));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());

        buffer.next();

        assert_matches!(buffer.peek(), Err(Eof(_)));
        assert_eq!(Location::new(15..15, 1, 16), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_err() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let actual: ScanResult<()> = buffer.consume(|_| {
            let err = ParserError::syntax(Location::new(0..0, 0, 0));
            Err(err)
        });

        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(actual) = actual.unwrap_err() else {
            unreachable!("already checked for Err(ScanErr(_))")
        };

        assert_eq!(&Syntax, actual.source());
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_ok() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let result = buffer.consume(|tok| Ok(Some(tok.clone())));
        assert_matches!(result, Ok(Identifier(_)));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_none() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let result: ScanResult<()> = buffer.consume(|_| None);
        assert_matches!(result, Err(NoMatch(_)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_some() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let result = buffer.consume(|tok| Some(tok.clone()));
        assert_matches!(result, Ok(Identifier(_)));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_false() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let result: ScanResult<TokenValue> = buffer.consume(|_| false);
        assert_matches!(result, Err(NoMatch(_)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_true() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let result = buffer.consume(|_| true);
        assert_matches!(result, Ok(Identifier(_)));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_peek2() {
        let mut buffer =  TokenStream::new("three identifiers innit", DEFAULT_CONFIG);

        let result = buffer.peek2();
        assert_matches!(result, (Ok(Identifier(_)), Ok(Identifier(_))));
        assert_eq!(Location::new(0..5, 1, 1), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, (Ok(Identifier(_)), Ok(Identifier(_))));
        assert_eq!(Location::new(6..17, 1, 7), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, (Ok(Identifier(_)), Err(Eof(_))));
        assert_eq!(Location::new(18..23, 1, 19), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, (Err(Eof(_)), Err(Eof(_))));
        assert_eq!(Location::new(23..23, 1, 24), buffer.current_location());
    }
}

use crate::parser::result::EofErrorKind::Eof;
use crate::parser::result::EofErrorKind::NotEof;
use crate::parser::result::EofResult;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanErrorKind::ScanErr;
use crate::parser::result::ScanResult;
use crate::parser::token_value::TokenValue;
use crate::parser::uescape_escape::uescape_escape;
use crate::parser::ParseResult;
use crate::parser::ParserConfig;
use crate::string_decoders::BasicStringDecoder;
use crate::string_decoders::ExtendedStringDecoder;
use crate::string_decoders::ExtendedStringResult;
use crate::string_decoders::UnicodeStringDecoder;
use elog::parser::ParserError;
use elog::parser::ParserErrorKind::ExtendedString;
use elog::parser::ParserErrorKind::InvalidUescapeDelimiter;
use elog::parser::ParserErrorKind::UescapeDelimiterMissing;
use elog::parser::ParserErrorKind::UnicodeString;
use elog::parser::ParserWarningKind;
use elog::HasLocation;
use postgres_basics::guc::BackslashQuote;
use postgres_basics::Located;
use postgres_basics::Location;
use postgres_basics::NumberRadix;
use postgres_basics::NAMEDATALEN;
use postgres_parser_ast::UnsignedNumber;
use postgres_parser_ast::UnsignedNumber::IntegerConst;
use postgres_parser_ast::UnsignedNumber::NumericConst;
use postgres_parser_lexer::BitStringKind;
use postgres_parser_lexer::IdentifierKind;
use postgres_parser_lexer::Keyword::Uescape;
use postgres_parser_lexer::Lexer;
use postgres_parser_lexer::RawTokenKind;
use postgres_parser_lexer::StringKind;
use std::collections::VecDeque;
