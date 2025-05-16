pub(crate) type ParseResult<T> = Result<T, ParserError>;

pub struct ParserResult {
    pub result: ParseResult<Vec<RawStmt>>,
    pub warnings: Vec<Located<ParserWarningKind>>,
}

pub struct Parser<'src> {
    buffer: TokenStream<'src>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        Self {
            buffer: TokenStream::new(source, config)
        }
    }

    /// Not reentrant (yet)!
    /// The TokenStream state is changed.
    pub fn parse(&mut self) -> ParserResult {

        let mut result = stmtmulti()
            .parse(&mut self.buffer)
            .required();

        // If it's not Eof, then something didn't match properly.
        // Discard the previous result, and mark the current location as a Syntax error.
        if !self.buffer.eof() {
            let loc = self.buffer.current_location();
            result = Err(ParserError::syntax(loc));
        }

        ParserResult {
            result,
            warnings: mem::take(self.buffer.warnings()),
        }
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::stmtmulti;
use crate::result::Required;
use crate::stream::TokenStream;
use crate::ParserConfig;
use elog::parser::ParserError;
use elog::parser::ParserWarningKind;
use postgres_basics::Located;
use postgres_parser_ast::RawStmt;
use std::mem;
