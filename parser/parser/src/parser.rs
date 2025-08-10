pub struct ParserResult {
    pub result: pg_elog::LocatedResult<Option<Vec<RawStmt>>>,
    pub warnings: Option<Vec<Located<parser::Warning>>>,
}

pub struct Parser<'src> {
    pub(crate) context: ParserContext<'src>,
}

impl<'src> Parser<'src> {

    pub fn new(source: &'src str, config: ParserConfig) -> Self {
        let stream = TokenStream::new(source, config);
        Self {
            context: ParserContext::new(stream, expr_list)
        }
    }

    /// Not reentrant (yet)!
    /// The TokenStream state is changed.
    pub fn parse(&mut self) -> ParserResult {

        let mut result = match stmtmulti(&mut self.context) {
            Ok(stmts) => Ok(Some(stmts)),
            Err(Eof(_)) => {
                // Empty input or no statements is valid.
                Ok(None)
            },
            Err(NoMatch(_)) => {
                // If it's not Eof, then something didn't match properly.
                // Mark the current location as a Syntax error.
                let loc = self.context.stream_mut().current_location();
                Err(syntax(loc))
            },
            Err(ScanErr(err)) => Err(err)
        };

        if !self.context.stream_mut().eof() {
            let loc = self.context.stream_mut().current_location();
            result = Err(syntax(loc));
        }

        let warnings = match self.context.stream_mut().warnings() {
            None => None,
            Some(warnings) => Some(mem::take(warnings))
        };

        ParserResult { result, warnings }
    }
}

use crate::combinators::expr_list;
use crate::combinators::stmtmulti;
use core::mem;
use pg_ast::RawStmt;
use pg_basics::Located;
use pg_combinators::ParserContext;
use pg_elog::parser;
use pg_parser_core::scan::Error::Eof;
use pg_parser_core::scan::Error::NoMatch;
use pg_parser_core::scan::Error::ScanErr;
use pg_parser_core::stream::TokenStream;
use pg_parser_core::syntax;
use pg_parser_core::ParserConfig;
