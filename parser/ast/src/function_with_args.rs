#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionWithArgs {
    name: QualifiedName,
    /// `None` if parameters weren't specified
    args: Option<Vec<FunctionParameter>>
}

impl FunctionWithArgs {
    pub fn new(name: QualifiedName, args: Option<Vec<FunctionParameter>>) -> Self {
        Self { name, args }
    }

    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    pub fn args(&self) -> &Option<Vec<FunctionParameter>> {
        &self.args
    }

    pub fn input_arguments(&self) -> Option<Vec<&FunctionParameter>> {

        let Some(args) = &self.args else { return None };

        let inputs = args.iter()
            .filter(|arg| matches!(arg.mode(), Default | In | Variadic))
            .collect();

        Some(inputs)
    }
}

use crate::FunctionParameter;
use crate::FunctionParameterMode::Default;
use crate::FunctionParameterMode::In;
use crate::FunctionParameterMode::Variadic;
use pg_basics::QualifiedName;
