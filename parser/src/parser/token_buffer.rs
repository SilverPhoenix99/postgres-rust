pub(super) struct TokenBuffer<'src> {
    lexer: Lexer<'src>,
    peeked: Option<Located<EofResult<TokenKind>>>
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
    pub fn eof(&mut self) -> bool {
        matches!(self.peeked().0, Err(EofErrorKind::Eof))
    }

    /// Returns the location of the current token,
    /// or an empty-length location if in the Eof state.
    #[inline(always)]
    pub fn current_location(&mut self) -> Location {
        self.peeked().1.clone()
    }

    #[inline(always)]
    pub fn next(&mut self) {
        self.peeked = None;
    }

    #[inline(always)]
    pub fn peek(&mut self) -> EofResult<TokenKind> {
        self.peeked().0.clone()
    }

    fn peeked(&mut self) -> &Located<EofResult<TokenKind>> {
        use EofErrorKind::*;

        self.peeked.get_or_insert_with(||
            match self.lexer.next() {
                None => {
                    let loc = self.lexer.current_location();
                    (Err(Eof), loc)
                },
                Some(Ok((tok, loc))) => (Ok(tok), loc),
                Some(Err(lex_err)) => {
                    let loc = lex_err.location().clone();
                    let err = lex_err.into();
                    (Err(NotEof(err)), loc)
                },
            }
        )
    }

    #[inline(always)]
    pub fn consume_eq(&mut self, kind: TokenKind) -> ScanResult<TokenKind> {
        self.consume(|tok| kind == tok)
    }

    #[inline(always)]
    pub fn consume_kw_eq(&mut self, keyword: Keyword) -> ScanResult<Keyword> {
        self.consume_kws(|kw| keyword == kw)
    }

    pub fn consume_kws(&mut self, pred: impl Fn(Keyword) -> bool) -> ScanResult<Keyword> {
        self.consume(|tok|
            tok.keyword().filter(|kw| pred(*kw))
        )
    }
}

pub(super) trait TokenConsumer<TOut, FRes> {
    fn consume<F>(&mut self, f: F) -> ScanResult<TOut>
    where
        F: Fn(TokenKind) -> FRes;
}

/// Consumers are not allowed to return `Err(None)` (Eof),
/// which is an internal error that's only returned by the `TokenBuffer` directly.
pub(super) type ConsumerResult<T> = ParseResult<Option<T>>;

impl<TOut> TokenConsumer<TOut, ConsumerResult<TOut>> for TokenBuffer<'_> {
    fn consume<F>(&mut self, mapper: F) -> ScanResult<TOut>
    where
        F: Fn(TokenKind) -> ConsumerResult<TOut>
    {
        let tok = match self.peek() {
            Ok(tok) => tok,
            Err(EofErrorKind::Eof) => return Err(ScanErrorKind::Eof),
            Err(NotEof(err)) => return Err(ScanErr(err)),
        };

        let Some(result) = mapper(tok).map_err(ScanErr)? else {
            return Err(NoMatch)
        };

        // The mapper matched the token.
        // Consume it from the Lexer.
        self.next();

        Ok(result)
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
    use crate::lexer::IdentifierKind::BasicIdentifier;
    use crate::parser::error::PartialParserError;
    use crate::parser::result::ScanErrorKind::NoMatch;
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

        let actual: ScanResult<()> = buffer.consume(|_| Err(
            PartialParserError::syntax(fn_info!(""))
        ));

        assert_matches!(actual, Err(ScanErr(_)));
        let ScanErr(actual) = actual.unwrap_err() else {
            unreachable!("already checked for Err(ScanErr(_))")
        };

        assert_eq!(&Syntax, actual.report().source());
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_ok() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(|tok| Ok(Some(tok)));
        assert_eq!(Ok(Identifier(BasicIdentifier)), result);
        assert_eq!(Location::new(4..15, 1, 5), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_none() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result: ScanResult<()> = buffer.consume(|_| None);
        assert_eq!(Err(NoMatch), result);
        assert_eq!(Location::new(0..3, 1, 1), buffer.current_location());
    }

    #[test]
    fn test_consume_returning_some() {
        let lexer = Lexer::new("two identifiers", true);
        let mut buffer =  TokenBuffer::new(lexer);

        let result = buffer.consume(Some);
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
            EofErrorKind::{self, NotEof},
            EofResult,
            ScanErrorKind::{self, NoMatch, ScanErr},
            ScanResult,
        },
        ParseResult,
    },
};
use postgres_basics::{Located, Location};
