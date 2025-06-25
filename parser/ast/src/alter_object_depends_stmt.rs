#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterObjectDependsStmt {
    target: AlterObjectDependsTarget,
    extension: Str,
    action: AddDrop,
}

impl AlterObjectDependsStmt {

    pub fn new<T: Into<Str>>(target: AlterObjectDependsTarget, extension: T, action: AddDrop) -> Self {
        Self {
            target,
            extension: extension.into(),
            action,
        }
    }

    pub fn target(&self) -> &AlterObjectDependsTarget {
        &self.target
    }

    pub fn extension(&self) -> &str {
        &self.extension
    }

    pub fn action(&self) -> AddDrop {
        self.action
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterObjectDependsTarget {
    Function(FunctionWithArgs),
    Index(QualifiedName),
    MaterializedView(QualifiedName),
    Procedure(FunctionWithArgs),
    Routine(FunctionWithArgs),
    Trigger { name: Str, table: QualifiedName },
}

use crate::AddDrop;
use crate::FunctionWithArgs;
use pg_basics::QualifiedName;
use pg_basics::Str;
