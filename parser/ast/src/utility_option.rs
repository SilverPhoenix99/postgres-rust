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

impl From<UtilityOptionName> for UtilityOption {
    fn from(name: UtilityOptionName) -> Self {
        Self::new(name, None)
    }
}

use crate::UtilityOptionName;
use crate::VarValue;
