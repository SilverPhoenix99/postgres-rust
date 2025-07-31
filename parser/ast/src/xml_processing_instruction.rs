#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlProcessingInstruction {
    name: Str,
    value: Option<ExprNode>,
}

impl XmlProcessingInstruction {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Str>,
    {
        Self {
            name: name.into(),
            value: None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_value(&mut self, value: Option<ExprNode>) -> &mut Self {
        self.value = value;
        self
    }

    pub fn with_value<T>(mut self, value: T) -> Self
    where
        T: Into<ExprNode>,
    {
        self.value = Some(value.into());
        self
    }

    pub fn value(&self) -> Option<&ExprNode> {
        self.value.as_ref()
    }
}

use crate::ExprNode;
use pg_basics::Str;
