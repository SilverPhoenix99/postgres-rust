#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionParameter {
    name: Option<Str>,
    mode: FunctionParameterMode,
    arg_type: FuncType,
}

impl FunctionParameter {

    pub fn new<T: Into<FuncType>>(arg_type: T) -> Self {
        Self {
            name: None,
            mode: Default::default(),
            arg_type: arg_type.into(),
        }
    }

    pub fn set_name(&mut self, name: Option<Str>) -> &mut Self {
        self.name = name;
        self
    }

    pub fn with_name<T: Into<Str>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_mode(&mut self, mode: FunctionParameterMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn with_mode(mut self, mode: FunctionParameterMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn mode(&self) -> FunctionParameterMode {
        self.mode
    }

    pub fn arg_type(&self) -> &FuncType {
        &self.arg_type
    }
}

impl<T: Into<FuncType>> From<T> for FunctionParameter {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

use crate::FuncType;
use crate::FunctionParameterMode;
use pg_basics::Str;
