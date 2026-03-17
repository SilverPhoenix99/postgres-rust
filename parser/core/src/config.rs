#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ParserConfig {
    backslash_quote: BackslashQuote,
}

impl ParserConfig {

    pub const fn new(backslash_quote: BackslashQuote) -> Self {
        Self { backslash_quote }
    }

    pub fn backslash_quote(&self) -> BackslashQuote {
        self.backslash_quote
    }
}

use pg_basics::guc::BackslashQuote;
