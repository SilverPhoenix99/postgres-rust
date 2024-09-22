use postgres_basics::guc::BackslashQuote;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum ParseMode {
    #[default]
    Default,
    TypeName,
    PlpgsqlExpr,
    PlpgsqlAssign1,
    PlpgsqlAssign2,
    PlpgsqlAssign3,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParserConfig {
    standard_conforming_strings: bool,
    backslash_quote: BackslashQuote,
    mode: ParseMode
}

impl ParserConfig {

    #[inline(always)]
    pub fn new(standard_conforming_strings: bool, backslash_quote: BackslashQuote, mode: ParseMode) -> Self {
        Self {
            standard_conforming_strings,
            backslash_quote,
            mode
        }
    }

    #[inline(always)]
    pub fn standard_conforming_strings(&self) -> bool {
        self.standard_conforming_strings
    }

    #[inline(always)]
    pub fn backslash_quote(&self) -> BackslashQuote {
        self.backslash_quote
    }

    #[inline(always)]
    pub fn mode(&self) -> ParseMode {
        self.mode
    }
}
