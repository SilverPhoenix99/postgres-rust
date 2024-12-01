#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncArg {
    name: Option<Str>,
    mode: FunctionParameterMode,
    arg_type: FuncType,
}

impl FuncArg {
    pub fn new(name: Option<Str>, mode: FunctionParameterMode, arg_type: FuncType) -> Self {
        Self { name, mode, arg_type }
    }

    pub fn name(&self) -> &Option<Str> {
        &self.name
    }

    pub fn mode(&self) -> FunctionParameterMode {
        self.mode
    }

    pub fn arg_type(&self) -> &FuncType {
        &self.arg_type
    }
}

use crate::parser::ast_node::FuncType;
use crate::parser::ast_node::FunctionParameterMode;
use postgres_basics::Str;
