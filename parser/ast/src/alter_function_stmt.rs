#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterFunctionStmt {
    kind: AlterFunctionKind,
    name: FunctionWithArgs,
    actions: Vec<AlterFunctionOption>
}

impl AlterFunctionStmt {

    pub fn new(kind: AlterFunctionKind, name: FunctionWithArgs, actions: Vec<AlterFunctionOption>) -> Self {
        Self { kind, name, actions }
    }

    pub fn kind(&self) -> AlterFunctionKind {
        self.kind
    }

    pub fn name(&self) -> &FunctionWithArgs {
        &self.name
    }

    pub fn actions(&self) -> &[AlterFunctionOption] {
        &self.actions
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlterFunctionKind {
    Function,
    Procedure,
    Routine,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AlterFunctionOption {
    Cost(SignedNumber),
    Leakproof(bool),
    Parallel(Str),
    Reset(VariableTarget),
    Rows(SignedNumber),
    Security(bool),
    Set(SetRestMore),
    Strict(bool),
    Support(QualifiedName),
    Volatility(Volatility),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Volatility {
    Immutable,
    Stable,
    Volatile,
}

use crate::FunctionWithArgs;
use pg_basics::QualifiedName;
use pg_basics::Str;
use pg_generic_set_ast::SetRestMore;
use pg_generic_set_ast::VariableTarget;
use pg_sink_ast::SignedNumber;
