#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UtilityOptionName {
    Analyze,
    Format,
    Generic(Str)
}

use pg_basics::Str;
