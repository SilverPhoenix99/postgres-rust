#[derive(Debug)]
pub struct TokenStream<'src> {
    lexer: BufferedLexer<'src>,
    buf: VecDeque<eof::Result<Located<TokenValue>>>
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

    pub fn source(&self) -> &'src str {
        self.lexer.lexer.source()
    }

    pub fn warnings(&mut self) -> Option<&mut Vec<Located<Warning>>> {
        if self.lexer.warnings.is_empty() {
            None
        }
        else {
            Some(&mut self.lexer.warnings)
        }
    }

    pub fn eof(&mut self) -> bool {
        matches!(self.peek_mut(), Err(Eof(_)))
    }

    /// Returns the location of the current token,
    /// or an empty-length location if in the Eof state.
    pub fn current_location(&mut self) -> Location {
        match self.peek_mut() {
            Ok((_, loc)) | Err(Eof(loc)) => loc.clone(),
            Err(NotEof(err)) => err.location().clone(),
        }
    }

    pub fn next(&mut self) {
        self.buf.pop_front();
    }

    pub fn skip(&mut self, n: usize) {

        if n == 0 {
            return;
        }

        for _ in 0..n {
            self.fill_buf();
            self.next();
        }
    }

    pub fn peek(&mut self) -> eof::Result<&TokenValue> {
        match self.peek_mut() {
            Ok((tok, _)) => Ok(tok),
            Err(err) => Err(err.clone()),
        }
    }

    fn peek_mut(&mut self) -> &mut eof::Result<Located<TokenValue>> {

        self.fill_buf();

        // SAFETY: `fill_buf()` always adds 2 elements to `self.buf`,
        //         even when the lexer is done
        self.buf.front_mut().unwrap()
    }

    /// Either returns both tokens, or the first error between the two.
    pub fn peek2(&mut self) -> eof::Result<(&TokenValue, &TokenValue)> {

        self.fill_buf();

        // SAFETY: `fill_buf()` always adds 2 elements to `self.buf`,
        //         even when the lexer is in Eof
        let first = self.buf.front()
            .expect("first element missing: `fill_buf()` should have filled 2 elements into `self.buf`");
        let second = self.buf.get(1)
            .expect("second element missing: `fill_buf()` should have filled 2 elements into `self.buf`");

        let first = match first {
            Ok((tok, _)) => tok,
            Err(err) => return Err(err.clone()),
        };

        let second = match second {
            Ok((tok, _)) => tok,
            Err(err) => return  Err(err.clone()),
        };

        Ok((first, second))
    }

    fn fill_buf(&mut self) {
        while self.buf.len() < 2 {
            let result = self.lex_next();
            self.buf.push_back(result);
        }
    }

    fn lex_next(&mut self) -> eof::Result<Located<TokenValue>> {

        let (tok, loc) = self.lexer.next()?;
        let slice = loc.slice(self.source());

        match tok {
            RawTokenKind::Operator(op) => Ok((TokenValue::Operator(op), loc)),
            RawTokenKind::Keyword(kw) => Ok((TokenValue::Keyword(kw), loc)),
            RawTokenKind::Param { index } => Ok((TokenValue::Param { index }, loc)),
            RawTokenKind::UserDefinedOperator => {
                let value = TokenValue::UserDefinedOperator(slice.into());
                Ok((value, loc))
            },
            RawTokenKind::NumberLiteral(radix) => {
                let value = parse_number(slice, radix);
                let value = TokenValue::UnsignedNumber(value);
                Ok((value, loc))
            },
            RawTokenKind::BitStringLiteral(kind) => self.lexer.parse_bit_string(slice, loc, kind),
            RawTokenKind::Identifier(kind) => self.lexer.parse_identifier(slice, loc, kind),
            RawTokenKind::StringLiteral(kind) => self.lexer.parse_string(slice, loc, kind),
        }
    }
}

impl<'src> From<&'src str> for TokenStream<'src> {
    fn from(value: &'src str) -> Self {
        TokenStream::new(value, ParserConfig::default())
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

pub trait TokenConsumer<TOut, FRes> {
    fn consume<F>(&mut self, f: F) -> scan::Result<TOut>
    where
        F: Fn(&mut TokenValue) -> FRes;
}

/// Consumers are not allowed to return `Err(Eof)`,
/// which is an internal error that's only returned by the `TokenBuffer` directly.
pub type LocatedResult<T> = pg_elog::LocatedResult<Option<T>>;

impl<TOut> TokenConsumer<TOut, LocatedResult<TOut>> for TokenStream<'_> {
    fn consume<F>(&mut self, mapper: F) -> scan::Result<TOut>
    where
        F: Fn(&mut TokenValue) -> LocatedResult<TOut>
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
    fn consume<F>(&mut self, mapper: F) -> scan::Result<TOut>
    where
        F: Fn(&mut TokenValue) -> Option<TOut>
    {
        self.consume(|tok| Ok(mapper(tok)))
    }
}

impl TokenConsumer<TokenValue, bool> for TokenStream<'_> {
    fn consume<P>(&mut self, pred: P) -> scan::Result<TokenValue>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::Error::ScanErr;
    use crate::syntax;
    use pg_elog::parser::Error::Syntax;
    use pg_elog::Error::Parser;
    use TokenValue::Identifier;

    #[test]
    fn test_eof() {
        let mut buffer =  TokenStream::from("");

        assert!(buffer.eof())
    }

    #[test]
    fn test_next_and_peek_and_current_location() {
        let mut buffer =  TokenStream::from("two identifiers");

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
        let mut buffer =  TokenStream::from("two identifiers");

        let actual: scan::Result<()> = buffer.consume(|_| {
            let err = syntax(Location::new(0..0, 0, 0));
            Err(err)
        });

        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(actual) = actual.unwrap_err() else {
            unreachable!("already checked for Err(ScanErr(_))")
        };

        assert_eq!(&Parser(Syntax), actual.source());
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_ok() {
        let mut buffer =  TokenStream::from("two identifiers");

        let result = buffer.consume(|tok| Ok(Some(tok.clone())));
        assert_matches!(result, Ok(Identifier(_)));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_none() {
        let mut buffer =  TokenStream::from("two identifiers");

        let result: scan::Result<()> = buffer.consume(|_| None);
        assert_matches!(result, Err(NoMatch(_)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_some() {
        let mut buffer =  TokenStream::from("two identifiers");

        let result = buffer.consume(|tok| Some(tok.clone()));
        assert_matches!(result, Ok(Identifier(_)));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_false() {
        let mut buffer =  TokenStream::from("two identifiers");

        let result: scan::Result<TokenValue> = buffer.consume(|_| false);
        assert_matches!(result, Err(NoMatch(_)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_true() {
        let mut buffer =  TokenStream::from("two identifiers");

        let result = buffer.consume(|_| true);
        assert_matches!(result, Ok(Identifier(_)));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_peek2() {
        let mut buffer =  TokenStream::from("three identifiers innit");

        let result = buffer.peek2();
        assert_matches!(result, Ok((Identifier(_), Identifier(_))));
        assert_eq!(Location::new(0..5, 1, 1), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, Ok((Identifier(_), Identifier(_))));
        assert_eq!(Location::new(6..17, 1, 7), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, Err(Eof(_)));
        assert_eq!(Location::new(18..23, 1, 19), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, Err(Eof(_)));
        assert_eq!(Location::new(23..23, 1, 24), buffer.current_location());
    }
}

use crate::eof;
use crate::eof::Error::Eof;
use crate::eof::Error::NotEof;
use crate::scan;
use crate::scan::Error::NoMatch;
use crate::scan::Error::ScanErr;
use crate::stream::buffered_lexer::BufferedLexer;
use crate::stream::TokenValue;
use crate::ParserConfig;
use alloc::collections::VecDeque;
use pg_basics::guc::BackslashQuote;
use pg_basics::Located;
use pg_basics::Location;
use pg_basics::NumberRadix;
use pg_basics::UnsignedNumber;
use pg_basics::UnsignedNumber::IntegerConst;
use pg_basics::UnsignedNumber::NumericConst;
use pg_elog::parser::Warning;
use pg_elog::HasLocation;
use pg_lexer::Lexer;
use pg_lexer::RawTokenKind;
