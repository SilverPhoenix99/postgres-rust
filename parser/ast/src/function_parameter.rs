#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionParameter {
    name: Option<Str>,
    mode: FunctionParameterMode,
    arg_type: FuncType,
}

impl FunctionParameter {
    pub fn new(name: Option<Str>, mode: FunctionParameterMode, arg_type: FuncType) -> Self {
        Self { name, mode, arg_type }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn mode(&self) -> FunctionParameterMode {
        self.mode
    }

    pub fn arg_type(&self) -> &FuncType {
        &self.arg_type
    }
}

impl From<FuncType> for FunctionParameter {
    fn from(value: FuncType) -> Self {
        Self::new(None, Default::default(), value)
    }
}

use crate::FuncType;
use crate::FunctionParameterMode;
use pg_basics::Str;
