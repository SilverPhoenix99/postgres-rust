#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtilityOption {
    name: UtilityOptionName,
    value: Option<VarValue>
}

impl UtilityOption {
    pub fn new(name: UtilityOptionName) -> Self {
        Self {
            name,
            value: None
        }
    }

    pub fn name(&self) -> &UtilityOptionName {
        &self.name
    }
    
    pub fn set_value(&mut self, value: Option<VarValue>) -> &mut Self {
        self.value = value;
        self
    }
    
    pub fn with_value<T: Into<VarValue>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn value(&self) -> Option<&VarValue> {
        self.value.as_ref()
    }
}

impl From<UtilityOptionName> for UtilityOption {
    fn from(name: UtilityOptionName) -> Self {
        Self::new(name)
    }
}

use crate::UtilityOptionName;
use crate::VarValue;
