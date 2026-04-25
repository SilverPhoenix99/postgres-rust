#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionWithArgs {
    name: QualifiedName,
    args: FuncArgs
}

impl FunctionWithArgs {
    pub fn new(name: QualifiedName, args: FuncArgs) -> Self {
        Self { name, args }
    }

    pub fn name(&self) -> &[Str] {
        &self.name
    }

    pub fn args(&self) -> &FuncArgs {
        &self.args
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FuncArgs {
    /// When parameters weren't specified, e.g., `func`.
    NoArgs,
    /// When parameters were specified, but the list is empty, e.g., `func()`.
    EmptyArgs,
    /// When parameters were specified and the list is not empty, e.g., `func(a, b)`.
    Args(Vec<FunctionParameter>)
}

use crate::FunctionParameter;
use pg_basics::QualifiedName;
use pg_basics::Str;
