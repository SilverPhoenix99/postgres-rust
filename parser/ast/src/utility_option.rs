#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtilityOption {
    name: UtilityOptionName,
    value: Option<VarValue>
}

impl UtilityOption {
    pub fn new(name: UtilityOptionName, value: Option<VarValue>) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &UtilityOptionName {
        &self.name
    }

    pub fn value(&self) -> Option<&VarValue> {
        self.value.as_ref()
    }
}

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

impl From<UtilityOptionName> for UtilityOption {
    fn from(name: UtilityOptionName) -> Self {
        Self::new(name, None)
    }
}

use pg_basics::Str;
use pg_generic_set_ast::VarValue;
