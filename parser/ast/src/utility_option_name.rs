#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UtilityOptionName {
    Analyze,
    Format,
    Generic(Str)
}

impl UtilityOptionName {
    pub fn with_value<T: Into<VarValue>>(self, value: T) -> UtilityOption {
        UtilityOption::new(self, Some(value.into()))
    }
}

use crate::UtilityOption;
use crate::VarValue;
use pg_basics::Str;
