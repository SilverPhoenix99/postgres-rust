pub enum ParserWarningKind {
    DeprecatedGlobalTemporaryTable,
    NonstandardEscape(ExtendedStringWarning),
}

impl ParserWarningKind {

    pub fn sqlstate(self) -> SqlState {
        match self {
            Self::DeprecatedGlobalTemporaryTable => SqlState::Warning,
            Self::NonstandardEscape(warn) => warn.sqlstate(),
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::DeprecatedGlobalTemporaryTable => "GLOBAL is deprecated in temporary table creation",
            Self::NonstandardEscape(warn) => warn.message()
        }
    }

    pub fn hint(self) -> Option<&'static str> {
        match self {
            Self::DeprecatedGlobalTemporaryTable => None,
            Self::NonstandardEscape(warn) => Some(warn.hint()),
        }
    }
}

use crate::string_decoders::ExtendedStringWarning;
use postgres_basics::sql_state::SqlState;
