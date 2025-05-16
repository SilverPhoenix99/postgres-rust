#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParserConfig {
    standard_conforming_strings: bool,
    backslash_quote: BackslashQuote,
}

impl ParserConfig {

    #[inline(always)]
    pub const fn new(standard_conforming_strings: bool, backslash_quote: BackslashQuote) -> Self {
        Self { standard_conforming_strings, backslash_quote }
    }

    #[inline(always)]
    pub fn standard_conforming_strings(&self) -> bool {
        self.standard_conforming_strings
    }

    #[inline(always)]
    pub fn backslash_quote(&self) -> BackslashQuote {
        self.backslash_quote
    }
}

use pg_basics::guc::BackslashQuote;
