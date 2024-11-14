pub(super) struct TokenBuffer<'src> {
    lexer: Lexer<'src>,
    buf: VecDeque<EofResult<Located<TokenKind>>>,
}

impl<'src> TokenBuffer<'src> {

    #[inline(always)]
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self {
            lexer,
            buf: VecDeque::with_capacity(2)
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &'src str {
        self.lexer.source()
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
    pub fn peek(&mut self) -> EofResult<TokenKind> {
        match self.peeked() {
            Ok((tok, _)) => Ok(*tok),
            Err(err) => Err(err.clone()),
        }
    }

    pub fn peek_with_slice(&mut self) -> EofResult<(TokenKind, &'src str)> {
        let source = self.lexer.source();
        match self.peeked() {
            Ok((tok, loc)) => {
                let slice = loc.slice(source);
                Ok((*tok, slice))
            },
            Err(err) => Err(err.clone()),
        }
    }

    pub fn peek2(&mut self) -> (EofResult<TokenKind>, EofResult<TokenKind>) {

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

    fn peeked(&mut self) -> &EofResult<Located<TokenKind>> {

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

    fn lex_next(&mut self) -> EofResult<Located<TokenKind>> {

        match self.lexer.next() {
            Some(Ok(tok)) => Ok(tok),
            Some(Err(lex_err)) => Err(NotEof(lex_err.into())),
            None => {
                let loc = self.lexer.current_location();
                Err(Eof(loc))
            },
        }
    }

    #[inline(always)]
    pub fn consume_eq(&mut self, kind: TokenKind) -> ScanResult<TokenKind> {
        self.consume(|tok| kind == tok)
    }

    #[inline(always)]
    pub fn consume_kw_eq(&mut self, keyword: Keyword) -> ScanResult<Keyword> {
        self.consume_kw(|kw| keyword == kw)
    }

    pub fn consume_kw(&mut self, pred: impl Fn(Keyword) -> bool) -> ScanResult<Keyword> {
        self.consume(|tok|
            tok.keyword().filter(|kw| pred(*kw))
        )
    }
}

pub(super) type SlicedToken<'src> = (TokenKind, &'src str, Location);

pub(super) trait SlicedTokenConsumer<'src, TOut, FRes> {
    fn consume_with_slice<F>(&mut self, f: F) -> ScanResult<TOut>
    where
        F: Fn(SlicedToken<'src>) -> FRes;
}

pub(super) trait TokenConsumer<TOut, FRes> {
    fn consume<F>(&mut self, f: F) -> ScanResult<TOut>
    where
        F: Fn(TokenKind) -> FRes;
}

/// Consumers are not allowed to return `Err(None)` (Eof),
/// which is an internal error that's only returned by the `TokenBuffer` directly.
pub(super) type ConsumerResult<T> = ParseResult<Option<T>>;

impl<'src, TOut> SlicedTokenConsumer<'src, TOut, ConsumerResult<TOut>> for TokenBuffer<'src> {
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

impl<'src, TOut> SlicedTokenConsumer<'src, TOut, Option<TOut>> for TokenBuffer<'src> {
    #[inline(always)]
    fn consume_with_slice<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(SlicedToken<'src>) -> Option<TOut>
    {
        self.consume_with_slice(|tok| Ok(mapper(tok)))
    }
}

impl<'src> SlicedTokenConsumer<'src, SlicedToken<'src>, bool> for TokenBuffer<'src> {
    #[inline(always)]
    fn consume_with_slice<P>(&mut self, pred: P) -> ScanResult<SlicedToken<'src>>
    where
        P: Fn(SlicedToken<'src>) -> bool
    {
        self.consume_with_slice(|tok| pred(tok.clone()).then_some(tok))
    }
}

impl<TOut> TokenConsumer<TOut, ConsumerResult<TOut>> for TokenBuffer<'_> {
    #[inline(always)]
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(TokenKind) -> ConsumerResult<TOut>
    {
        self.consume_with_slice(|(tok, _, _)| mapper(tok))
    }
}

impl<TOut> TokenConsumer<TOut, Option<TOut>> for TokenBuffer<'_> {
    #[inline(always)]
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(TokenKind) -> Option<TOut>
    {
        self.consume(|tok| Ok(mapper(tok)))
    }
}

impl TokenConsumer<TokenKind, bool> for TokenBuffer<'_> {
    #[inline(always)]
    fn consume<P>(&mut self, pred: P) -> ScanResult<TokenKind>
    where
        P: Fn(TokenKind) -> bool
    {
        self.consume(|tok| pred(tok).then_some(tok))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::IdentifierKind::Basic;
    use crate::parser::result::ScanErrorKind;
    use crate::parser::ParserError;
    use crate::parser::ParserErrorKind::Syntax;
    use postgres_basics::fn_info;
    use TokenKind::Identifier;

    #[test]
    fn test_eof() {
        let lexer = Lexer::new("", true);
        let mut buffer =  TokenBuffer::new(lexer);

        assert!(buffer.eof())
    }

    #[test]
    fn test_next_and_peek_and_current_location() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

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
    fn test_consume_eq() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        assert_matches!(buffer.consume_eq(TokenKind::Comma), Err(NoMatch(_)));

        assert_eq!(
            Ok(Identifier(Basic)),
            buffer.consume_eq(Identifier(Basic))
        );
    }

    #[test]
    fn test_consume_returning_err() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let actual: ScanResult<()> = buffer.consume(|_| {
            let err = ParserError::syntax(fn_info!(""), Location::new(0..0, 0, 0));
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
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|tok| Ok(Some(tok)));
        assert_eq!(Ok(Identifier(Basic)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_none() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result: ScanResult<()> = buffer.consume(|_| None);
        assert_matches!(result, Err(NoMatch(_)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_some() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(Some);
        assert_eq!(Ok(Identifier(Basic)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_false() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result: ScanResult<TokenKind> = buffer.consume(|_| false);
        assert_matches!(result, Err(NoMatch(_)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_true() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|_| true);
        assert_eq!(Ok(Identifier(Basic)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_peek2() {
        let lexer = Lexer::new("three identifiers innit", true);
        let mut buffer =  TokenBuffer::new(lexer);

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

use crate::{
    error::HasLocation,
    lexer::{Keyword, Lexer, TokenKind},
    parser::{
        result::{
            EofErrorKind::{Eof, NotEof},
            EofResult,
            ScanErrorKind::{NoMatch, ScanErr},
            ScanResult,
        },
        ParseResult,
    },
};
use postgres_basics::{Located, Location};
use std::collections::VecDeque;
