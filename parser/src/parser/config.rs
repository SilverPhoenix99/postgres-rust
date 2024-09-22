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
    pub standard_conforming_strings: bool,
    pub backslash_quote: BackslashQuote,
    pub mode: ParseMode
}

impl ParserConfig {

    pub fn new(standard_conforming_strings: bool, backslash_quote: BackslashQuote, mode: ParseMode) -> Self {
        Self {
            standard_conforming_strings,
            backslash_quote,
            mode
        }
    }
}
