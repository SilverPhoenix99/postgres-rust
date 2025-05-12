pub mod ast_node;
mod combinators;
mod config;
mod error;
mod result;
mod token_stream;
mod token_value;
mod uescape_escape;
mod warning;

pub use self::{
    config::ParserConfig,
    error::{ParserError, ParserErrorKind},
    warning::ParserWarningKind,
};

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
            result = Err(syntax_err(loc));
        }

        ParserResult {
            result,
            warnings: mem::take(self.buffer.warnings()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ParserConfig;
    use postgres_basics::guc::BackslashQuote::SafeEncoding;

    pub(super) static DEFAULT_CONFIG: ParserConfig = ParserConfig::new(true, SafeEncoding);

    macro_rules! test_parser {
        (
            source = $source:expr,
            parser = $parser:expr,
            expected = $expected:expr
        ) => {{
            test_parser!($source, $parser, $expected)
        }};

        (
            $source:expr,
            $parser:expr,
            $expected:expr
        ) => {{
            use crate::parser::tests::DEFAULT_CONFIG;
            use crate::parser::token_stream::TokenStream;

            let mut stream = TokenStream::new($source, DEFAULT_CONFIG);
            let actual = $parser.parse(&mut stream);

            let expected = $expected;

            assert_eq!(Ok(expected), actual);
        }};
    }

    pub(super) use test_parser;
}

use crate::parser::ast_node::RawStmt;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::stmtmulti;
use crate::parser::error::syntax_err;
use crate::parser::result::Required;
use crate::parser::token_stream::TokenStream;
use postgres_basics::Located;
use std::mem;
