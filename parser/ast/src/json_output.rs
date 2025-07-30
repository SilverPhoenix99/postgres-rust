#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonOutput {
    type_name: Type,
    format: JsonFormat,
}

impl JsonOutput {
    pub fn new<T>(type_name: T) -> Self
    where
        T: Into<Type>,
    {
        Self {
            type_name: type_name.into(),
            format: JsonFormat::default()
        }
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }

    pub fn with_format(mut self, format: JsonFormat) -> Self {
        self.format = format;
        self
    }

    pub fn format(&self) -> JsonFormat {
        self.format
    }
}

impl<T> From<T> for JsonOutput
where
    T: Into<Type>,
{
    fn from(type_name: T) -> Self {
        Self::new(type_name)
    }
}

use crate::JsonFormat;
use crate::Type;
