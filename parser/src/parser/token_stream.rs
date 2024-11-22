#[derive(Debug)]
pub(super) struct TokenStream<'src> {
    backslash_quote: BackslashQuote,
    lexer: Lexer<'src>,
    buf: VecDeque<EofResult<Located<RawTokenKind>>>,
    /// All the warnings that have been collected while parsing.
    warnings: Vec<Located<ParserWarningKind>>
}

impl<'src> TokenStream<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        let lexer = Lexer::new(source, config.standard_conforming_strings());
        Self::with_lexer(lexer, config.backslash_quote())
    }

    pub fn with_lexer(lexer: Lexer<'src>, backslash_quote: BackslashQuote) -> Self {
        Self {
            backslash_quote,
            lexer,
            buf: VecDeque::with_capacity(2),
            warnings: Vec::new()
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &'src str {
        self.lexer.source()
    }

    pub fn backslash_quote(&self) -> BackslashQuote {
        self.backslash_quote
    }

    pub fn warnings(&mut self) -> &mut Vec<Located<ParserWarningKind>> {
        &mut self.warnings
    }

    #[inline(always)]
    pub fn eof(&mut self) -> bool {
        matches!(self.peeked(), Err(Eof(_)))
    }

    /// Returns the location of the current token,
    /// or an empty-length location if in the Eof state.
    #[inline(always)]
    pub fn current_location(&mut self) -> Location {
        match self.peeked() {
            Ok((_, loc)) | Err(Eof(loc)) => loc.clone(),
            Err(NotEof(err)) => err.location().clone(),
        }
    }

    pub fn slice(&mut self) -> Option<&'src str> {

        let source = self.source();

        let Ok((_, loc)) = self.peeked() else {
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
    pub fn peek(&mut self) -> EofResult<RawTokenKind> {
        match self.peeked() {
            Ok((tok, _)) => Ok(*tok),
            Err(err) => Err(err.clone()),
        }
    }

    pub fn peek_with_slice(&mut self) -> EofResult<(RawTokenKind, &'src str)> {
        let source = self.lexer.source();
        match self.peeked() {
            Ok((tok, loc)) => {
                let slice = loc.slice(source);
                Ok((*tok, slice))
            },
            Err(err) => Err(err.clone()),
        }
    }

    pub fn peek2(&mut self) -> (EofResult<RawTokenKind>, EofResult<RawTokenKind>) {

        self.fill_buf();

        // SAFETY: `fill_buf()` always adds 2 elements to `self.buf`,
        //         even when the lexer is in Eof
        let first = self.buf.front()
            .expect("first element missing: `fill_buf()` should have filled 2 elements into `self.buf`");
        let second = self.buf.get(1)
            .expect("second element missing: `fill_buf()` should have filled 2 elements into `self.buf`");

        let first = match first {
            Ok((tok, _)) => Ok(*tok),
            Err(err) => Err(err.clone()),
        };

        let second = match second {
            Ok((tok, _)) => Ok(*tok),
            Err(err) => Err(err.clone()),
        };

        (first, second)
    }

    fn peeked(&mut self) -> &EofResult<Located<RawTokenKind>> {

        self.fill_buf();

        // SAFETY: `fill_buf()` always adds 2 elements to `self.buf`,
        //         even when the lexer is done
        self.buf.front().unwrap()
    }

    fn fill_buf(&mut self) {
        while self.buf.len() < 2 {
            let result = self.lex_next();
            self.buf.push_back(result);
        }
    }

    fn lex_next(&mut self) -> EofResult<Located<RawTokenKind>> {

        match self.lexer.next() {
            Some(Ok(tok)) => Ok(tok),
            Some(Err(lex_err)) => Err(NotEof(lex_err.into())),
            None => {
                let loc = self.lexer.current_location();
                Err(Eof(loc))
            },
        }
    }
}

pub(super) type SlicedToken<'src> = (RawTokenKind, &'src str, Location);

pub(super) trait SlicedTokenConsumer<'src, TOut, FRes> {
    fn consume_with_slice<F>(&mut self, f: F) -> ScanResult<TOut>
    where
        F: Fn(SlicedToken<'src>) -> FRes;
}

pub(super) trait TokenConsumer<TOut, FRes> {
    fn consume<F>(&mut self, f: F) -> ScanResult<TOut>
    where
        F: Fn(RawTokenKind) -> FRes;
}

/// Consumers are not allowed to return `Err(Eof)`,
/// which is an internal error that's only returned by the `TokenBuffer` directly.
pub(super) type ConsumerResult<T> = ParseResult<Option<T>>;

impl<'src, TOut> SlicedTokenConsumer<'src, TOut, ConsumerResult<TOut>> for TokenStream<'src> {
    fn consume_with_slice<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(SlicedToken<'src>) -> ConsumerResult<TOut>
    {
        let source = self.lexer.source();
        let tok = match self.peeked() {
            Ok((tok, loc)) => (*tok, loc.slice(source), loc.clone()),
            Err(err) => return Err(err.clone().into()),
        };

        let loc = tok.2.clone();
        let Some(result) = mapper(tok).map_err(ScanErr)? else {
            return Err(NoMatch(loc))
        };

        // The mapper matched the token.
        // Consume it from the Lexer.
        self.next();

        Ok(result)
    }
}

impl<'src, TOut> SlicedTokenConsumer<'src, TOut, Option<TOut>> for TokenStream<'src> {
    #[inline(always)]
    fn consume_with_slice<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(SlicedToken<'src>) -> Option<TOut>
    {
        self.consume_with_slice(|tok| Ok(mapper(tok)))
    }
}

impl<'src> SlicedTokenConsumer<'src, SlicedToken<'src>, bool> for TokenStream<'src> {
    #[inline(always)]
    fn consume_with_slice<P>(&mut self, pred: P) -> ScanResult<SlicedToken<'src>>
    where
        P: Fn(SlicedToken<'src>) -> bool
    {
        self.consume_with_slice(|tok| pred(tok.clone()).then_some(tok))
    }
}

impl<TOut> TokenConsumer<TOut, ConsumerResult<TOut>> for TokenStream<'_> {
    #[inline(always)]
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(RawTokenKind) -> ConsumerResult<TOut>
    {
        self.consume_with_slice(|(tok, _, _)| mapper(tok))
    }
}

impl<TOut> TokenConsumer<TOut, Option<TOut>> for TokenStream<'_> {
    #[inline(always)]
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(RawTokenKind) -> Option<TOut>
    {
        self.consume(|tok| Ok(mapper(tok)))
    }
}

impl TokenConsumer<RawTokenKind, bool> for TokenStream<'_> {
    #[inline(always)]
    fn consume<P>(&mut self, pred: P) -> ScanResult<RawTokenKind>
    where
        P: Fn(RawTokenKind) -> bool
    {
        self.consume(|tok| pred(tok).then_some(tok))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::IdentifierKind::Basic;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::ParserError;
    use crate::parser::ParserErrorKind::Syntax;
    use RawTokenKind::Identifier;

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

        let result = buffer.consume(|tok| Ok(Some(tok)));
        assert_eq!(Ok(Identifier(Basic)), result);
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

        let result = buffer.consume(Some);
        assert_eq!(Ok(Identifier(Basic)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_false() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let result: ScanResult<RawTokenKind> = buffer.consume(|_| false);
        assert_matches!(result, Err(NoMatch(_)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_true() {
        let mut buffer =  TokenStream::new("two identifiers", DEFAULT_CONFIG);

        let result = buffer.consume(|_| true);
        assert_eq!(Ok(Identifier(Basic)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_peek2() {
        let mut buffer =  TokenStream::new("three identifiers innit", DEFAULT_CONFIG);

        let result = buffer.peek2();
        assert_eq!((Ok(Identifier(Basic)), Ok(Identifier(Basic))), result);
        assert_eq!(Location::new(0..5, 1, 1), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_eq!((Ok(Identifier(Basic)), Ok(Identifier(Basic))), result);
        assert_eq!(Location::new(6..17, 1, 7), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, (Ok(Identifier(Basic)), Err(Eof(_))));
        assert_eq!(Location::new(18..23, 1, 19), buffer.current_location());

        buffer.next();
        let result = buffer.peek2();
        assert_matches!(result, (Err(Eof(_)), Err(Eof(_))));
        assert_eq!(Location::new(23..23, 1, 24), buffer.current_location());
    }
}

use crate::error::HasLocation;
use crate::lexer::Lexer;
use crate::lexer::RawTokenKind;
use crate::parser::result::EofErrorKind::Eof;
use crate::parser::result::EofErrorKind::NotEof;
use crate::parser::result::EofResult;
use crate::parser::result::ScanErrorKind::NoMatch;
use crate::parser::result::ScanErrorKind::ScanErr;
use crate::parser::result::ScanResult;
use crate::parser::ParseResult;
use crate::parser::ParserConfig;
use crate::parser::ParserWarningKind;
use postgres_basics::guc::BackslashQuote;
use postgres_basics::Located;
use postgres_basics::Location;
use std::collections::VecDeque;
