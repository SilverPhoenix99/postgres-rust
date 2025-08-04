#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncCall {
    name: QualifiedName,
    args: FuncArgsKind,
}

impl FuncCall {
    pub fn new(name: QualifiedName, args: FuncArgsKind) -> Self {
        Self { name, args }
    }

    pub fn name(&self) -> &[Str] {
        &self.name
    }

    pub fn args(&self) -> &FuncArgsKind {
        &self.args
    }
}

use crate::FuncArgsKind;
use pg_basics::QualifiedName;
use pg_basics::Str;
