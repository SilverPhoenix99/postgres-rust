/// GUC supports these types of variables:
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigType {
    Bool,
    Int,
    Real,
    String,
    Enum
}

impl Named for ConfigType {
    /**
     * Displayable names for GUC variable types (enum config_type)
     *
     * Note: these strings are deliberately not localized.
     */
    fn name(&self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Int => "integer",
            Self::Real => "real",
            Self::String => "string",
            Self::Enum => "enum",
        }
    }
}

use pg_basics::Named;
