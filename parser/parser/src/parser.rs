pub struct ParserResult {
    pub result: LocatedResult<Option<Vec<RawStmt>>>,
    pub warnings: Option<Vec<Located<Warning>>>,
}

pub struct Parser<'src> {
    pub(crate) stream: TokenStream<'src>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        Self {
            stream: TokenStream::new(source, config)
        }
    }

    /// Not reentrant (yet)!
    /// The TokenStream state is changed.
    pub fn parse(&mut self) -> ParserResult {

        let mut result = stmtmulti()
            .parse(&mut self.stream)
            .required();

        // If it's not Eof, then something didn't match properly.
        // Discard the previous result, and mark the current location as a Syntax error.
        if !self.stream.eof() {
            let loc = self.stream.current_location();
            result = Err(syntax(loc));
        }

        let warnings = match self.stream.warnings() {
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
use core::mem;
use pg_ast::RawStmt;
use pg_basics::Located;
use pg_elog::parser::Warning;
use pg_elog::syntax;
use pg_elog::LocatedResult;
