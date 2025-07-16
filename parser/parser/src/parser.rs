pub struct ParserResult {
    pub result: pg_elog::LocatedResult<Option<Vec<RawStmt>>>,
    pub warnings: Option<Vec<Located<parser::Warning>>>,
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

        let mut result = match stmtmulti(&mut self.stream) {
            Ok(stmts) => Ok(Some(stmts)),
            Err(Eof(_)) => {
                // Empty input or no statements is valid.
                Ok(None)
            },
            Err(NoMatch(_)) => {
                // If it's not Eof, then something didn't match properly.
                // Mark the current location as a Syntax error.
                let loc = self.stream.current_location();
                Err(syntax(loc))
            },
            Err(ScanErr(err)) => Err(err)
        };

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

use crate::combinators::stmtmulti;
use crate::scan::Error::Eof;
use crate::scan::Error::NoMatch;
use crate::scan::Error::ScanErr;
use crate::stream::TokenStream;
use crate::syntax;
use crate::ParserConfig;
use core::mem;
use pg_ast::RawStmt;
use pg_basics::Located;
use pg_elog::parser;
