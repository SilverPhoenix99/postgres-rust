pub(super) struct TokenBuffer<'src> {
    lexer: Lexer<'src>,
    peeked: Option<Option<LexerResult>>
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
        match self.peek() {
            Some(Ok((_, loc))) => loc.clone(),
            Some(Err(err)) => err.location().clone(),
            None => self.lexer.current_location(),
        }
    }

    #[inline(always)]
    pub fn next(&mut self) {
        self.peeked = None;
    }

    #[inline(always)]
    pub fn peek(&mut self) -> &Option<LexerResult> {
        self.peeked.get_or_insert_with(|| self.lexer.next())
    }

    /// Similar to `consume`, but tries to match one of many `mappers`.
    ///
    /// The `mappers` are executed in order, until the first that returns `Ok(Some(T))`.
    ///
    /// Iteration stops when a mapper returns either `Ok(Some(T))` or `Err(_)`.
    ///
    /// When a mapper doesn't match anything, then it should return `Ok(None)`,
    /// to signal that iteration should continue to the next mapper.
    #[inline]
    pub fn consume_any<T>(
        &mut self,
        mappers: &[&dyn Fn(&TokenKind) -> ConsumerResult<T>]
    ) -> OptResult<T>
    {
        self.consume(|tok| {

            for mapper in mappers {
                match mapper(tok) {
                    Ok(None) => {/* none matched, so try the next one */},
                    result => return result,
                }
            }

            // No mapper matched
            Ok(None)
        })
    }

    #[inline(always)]
    pub fn consume_eq(&mut self, kind: TokenKind) -> OptResult<TokenKind> {
        self.consume(|tok| kind.eq(tok))
    }

    #[inline(always)]
    pub fn consume_kw_eq(&mut self, keyword: Keyword) -> OptResult<&'static KeywordDetails> {
        self.consume(|tok|
            tok.keyword().filter(|details| details.keyword() == keyword)
        )
    }
}

/// Consumers are not allowed to return `Err(None)` (Eof),
/// which is an internal error that's only returned by the `TokenBuffer` directly.
pub(super) type ConsumerResult<T> = Result<Option<T>, ParserErrorKind>;

pub(super) trait TokenConsumer<TOut, FRes> {
    fn consume<F>(&mut self, f: F) -> OptResult<TOut>
    where
        F: Fn(&TokenKind) -> FRes;
}

impl<TOut> TokenConsumer<TOut, ConsumerResult<TOut>> for TokenBuffer<'_> {

    fn consume<F>(&mut self, mapper: F) -> OptResult<TOut>
    where
        F: Fn(&TokenKind) -> ConsumerResult<TOut>
    {
        match self.peek() {

            // Eof never matches
            None => Err(None),

            Some(Ok((tok, _))) => {
                match mapper(tok) {

                    // The mapper matched the token.
                    // Consume it from the Lexer.
                    Ok(Some(result)) => {
                        self.next();
                        Ok(Some(result))
                    }

                    // The mapper didn't match.
                    // Keep the token in the Lexer.
                    Ok(None) => Ok(None),

                    // Some error is present
                    Err(err) => Err(Some(err)),
                }
            },

            Some(Err(lex_err)) => {
                Err(Some(lex_err.clone().into()))
            }
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
    fn consume<F>(&mut self, mapper: F) -> OptResult<TOut>
    where
        F: Fn(&TokenKind) -> Option<TOut>
    {
        self.consume(|tok| {
            match mapper(tok) {
                Some(result) => Ok(Some(result)),
                None => Ok(None),
            }
        })
    }
}

impl<T> TokenConsumer<TokenKind, bool> for T
where
    T: TokenConsumer<TokenKind, ConsumerResult<TokenKind>>
{
    #[inline(always)]
    fn consume<P>(&mut self, pred: P) -> OptResult<TokenKind>
    where
        P: Fn(&TokenKind) -> bool
    {
        self.consume(|tok|
            if pred(tok) {
                Ok(Some(*tok))
            }
            else {
                Ok(None)
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::IdentifierKind::BasicIdentifier;
    use crate::parser::Lexer;
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

        assert_matches!(buffer.peek(), Some(Ok((_, _))));
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());

        buffer.next();

        assert_matches!(buffer.peek(), Some(Ok((_, _))));
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());

        buffer.next();

        assert_matches!(buffer.peek(), None);
        assert_eq!(Location::new(15..15, 1, 16), buffer.current_location());
    }

    #[test]
    fn test_consume_eq() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        assert_eq!(Ok(None), buffer.consume_eq(TokenKind::Comma));

        assert_eq!(
            Ok(Some(Identifier(BasicIdentifier))),
            buffer.consume_eq(Identifier(BasicIdentifier))
        );
    }

    #[test]
    fn test_consume_returning_err() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result: OptResult<TokenKind> = buffer.consume(|_| Err(Syntax));
        assert_eq!(Err(Some(Syntax)), result);
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_ok() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|tok| Ok(Some(*tok)));
        assert_eq!(Ok(Some(Identifier(BasicIdentifier))), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_none() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result: OptResult<TokenKind> = buffer.consume(|_| None);
        assert_eq!(Ok(None), result);
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_some() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|tok| Some(*tok));
        assert_eq!(Ok(Some(Identifier(BasicIdentifier))), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_false() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|_| false);
        assert_eq!(Ok(None), result);
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_true() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|_| true);
        assert_eq!(Ok(Some(Identifier(BasicIdentifier))), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }
}

use crate::error::HasLocation;
use crate::lexer::{Keyword, KeywordDetails, Lexer, LexerResult, TokenKind};
use crate::parser::{OptResult, ParserErrorKind};
use postgres_basics::Location;
