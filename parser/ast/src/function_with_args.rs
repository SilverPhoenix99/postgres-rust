#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionWithArgs {
    name: QualifiedName,
    /// * `None` if parameters weren't specified.
    /// * `Some(None)` if parameters were specified, but the list is empty, e.g., `func()`.
    /// * `Some(Some(vec![...]))` if parameters were specified and the list is not empty, e.g., `func(a, b)`.
    args: Option<Option<Vec<FunctionParameter>>>
}

impl FunctionWithArgs {
    pub fn new(name: QualifiedName, args: Option<Option<Vec<FunctionParameter>>>) -> Self {
        Self { name, args }
    }

    pub fn name(&self) -> &[Str] {
        &self.name
    }

    pub fn args(&self) -> Option<Option<&[FunctionParameter]>> {
        self.args.as_ref().map(Option::as_deref)
    }
}

use crate::FunctionParameter;
use pg_basics::QualifiedName;
use pg_basics::Str;
