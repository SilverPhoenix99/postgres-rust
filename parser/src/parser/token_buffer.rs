pub(super) struct TokenBuffer<'src> {
    lexer: Lexer<'src>,
    peeked: Option<EofResult<Located<TokenKind>>>
}

impl<'src> TokenBuffer<'src> {

    #[inline(always)]
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self { lexer, peeked: None }
    }

    #[inline(always)]
    pub fn source(&self) -> &'src str {
        self.lexer.source()
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.lexer.eof()
    }

    /// Returns the location of the current token,
    /// or an empty-length location if in the Eof state.
    #[inline(always)]
    pub fn current_location(&mut self) -> Location {
        use EofErrorKind::*;

        match self.peek() {
            Ok((_, loc)) => loc.clone(),
            Err(Eof) => self.lexer.current_location(),
            Err(ParserErr(ParserErrorKind::Lexer(lex_err))) => lex_err.location().clone(),
            Err(ParserErr(err)) => {
                panic!("peek() should only return `LexerError`, but actually returned {err}")
            }
        }
    }

    #[inline(always)]
    pub fn next(&mut self) {
        self.peeked = None;
    }

    #[inline(always)]
    pub fn peek(&mut self) -> &EofResult<Located<TokenKind>> {
        use EofErrorKind::*;

        self.peeked.get_or_insert_with(||
            match self.lexer.next() {
                None => Err(Eof),
                Some(Ok(tok)) => Ok(tok),
                Some(Err(lex_err)) => Err(ParserErr(lex_err.into())),
            }
        )
    }

    #[inline(always)]
    pub fn consume_eq(&mut self, kind: TokenKind) -> ScanResult<TokenKind> {
        self.consume(|tok| kind.eq(tok))
    }

    #[inline(always)]
    pub fn consume_kw_eq(&mut self, keyword: Keyword) -> ScanResult<Keyword> {
        self.consume_kws(|kw| keyword.eq(kw))
    }

    pub fn consume_kws(&mut self, pred: impl Fn(&Keyword) -> bool) -> ScanResult<Keyword> {
        self.consume(|tok|
            tok.keyword().filter(&pred)
        )
    }
}

/// Consumers are not allowed to return `Err(None)` (Eof),
/// which is an internal error that's only returned by the `TokenBuffer` directly.
pub(super) type ConsumerResult<T> = Result<Option<T>, ParserErrorKind>;

pub(super) trait TokenConsumer<TOut, FRes> {
    fn consume<F>(&mut self, f: F) -> ScanResult<TOut>
    where
        F: Fn(&TokenKind) -> FRes;
}

impl<TOut> TokenConsumer<TOut, ConsumerResult<TOut>> for TokenBuffer<'_> {

    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(&TokenKind) -> ConsumerResult<TOut>
    {
        let tok = match self.peek() {
            Ok((tok, _)) => tok,
            Err(err) => {
                let err = err.clone();
                return Err(err.into())
            }
        };

        match mapper(tok) {

            // The mapper matched the token.
            // Consume it from the Lexer.
            Ok(Some(result)) => {
                self.next();
                Ok(result)
            }

            // The mapper didn't match.
            // Keep the token in the Lexer.
            Ok(None) => Err(NoMatch),

            // Some error is present
            Err(err) => Err(err.into()),
        }
    }
}

impl<T, TOut> TokenConsumer<TOut, Option<TOut>> for T
where
    T: TokenConsumer<TOut, ConsumerResult<TOut>>
{
    /// Similar to `consume() -> OptResult`, but maps `None` to `Ok(None)`.
    /// Use this method when the consumption doesn't require returning errors.
    #[inline(always)]
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(&TokenKind) -> Option<TOut>
    {
        self.consume(|tok| Ok(mapper(tok)))
    }
}

impl<T> TokenConsumer<TokenKind, bool> for T
where
    T: TokenConsumer<TokenKind, ConsumerResult<TokenKind>>
{
    #[inline(always)]
    fn consume<P>(&mut self, pred: P) -> ScanResult<TokenKind>
    where
        P: Fn(&TokenKind) -> bool
    {
        self.consume(|tok| Ok(pred(tok).then_some(*tok)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::IdentifierKind::BasicIdentifier;
    use crate::parser::ParserErrorKind::Syntax;
    use TokenKind::Identifier;

    #[test]
    fn test_eof() {
        let lexer = Lexer::new("", true);
        let buffer =  TokenBuffer::new(lexer);

        assert!(buffer.eof())
    }

    #[test]
    fn test_next_and_peek_and_current_location() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        assert_matches!(buffer.peek(), Ok((_, _)));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());

        buffer.next();

        assert_matches!(buffer.peek(), Ok((_, _)));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());

        buffer.next();

        assert_matches!(buffer.peek(), Err(EofErrorKind::Eof));
        assert_eq!(Location::new(15..15, 1, 16), buffer.current_location());
    }

    #[test]
    fn test_consume_eq() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        assert_eq!(Err(NoMatch), buffer.consume_eq(TokenKind::Comma));

        assert_eq!(
            Ok(Identifier(BasicIdentifier)),
            buffer.consume_eq(Identifier(BasicIdentifier))
        );
    }

    #[test]
    fn test_consume_returning_err() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result: ScanResult<TokenKind> = buffer.consume(|_| Err(Syntax));
        assert_eq!(Err(ScanErrorKind::ParserErr(Syntax)), result);
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_ok() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|tok| Ok(Some(*tok)));
        assert_eq!(Ok(Identifier(BasicIdentifier)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_none() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result: ScanResult<TokenKind> = buffer.consume(|_| None);
        assert_eq!(Err(NoMatch), result);
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_some() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|tok| Some(*tok));
        assert_eq!(Ok(Identifier(BasicIdentifier)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_false() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|_| false);
        assert_eq!(Err(NoMatch), result);
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_true() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|_| true);
        assert_eq!(Ok(Identifier(BasicIdentifier)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }
}

use crate::{
    error::HasLocation,
    lexer::{Keyword, Lexer, TokenKind},
    parser::{
        result::{
            EofErrorKind,
            EofResult,
            ScanErrorKind::{self, NoMatch},
            ScanResult
        },
        ParserErrorKind
    }
};
use postgres_basics::{Located, Location};
