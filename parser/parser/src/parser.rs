pub(crate) type ParseResult<T> = Result<T, PgError>;

pub struct ParserResult {
    pub result: ParseResult<Option<Vec<RawStmt>>>,
    pub warnings: Option<Vec<Located<ParserWarningKind>>>,
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
            result = Err(syntax(loc));
        }
        
        let warnings = match self.buffer.warnings() {
            None => None,
            Some(warnings) => Some(mem::take(warnings))
        };

        ParserResult { result, warnings }
    }
}

use crate::combinators::foundation::Combinator;
use crate::combinators::stmtmulti;
use crate::result::Required;
use crate::stream::TokenStream;
use crate::ParserConfig;
use pg_ast::RawStmt;
use pg_basics::Located;
use pg_elog::syntax;
use pg_elog::ParserWarningKind;
use pg_elog::PgError;
use std::mem;
