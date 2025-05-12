#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterFunctionStmt {
    kind: AlterFunctionKind,
    name: FunctionWithArgs,
    actions: Vec<AlterFunctionOption>
}

impl AlterFunctionStmt {
    #[inline(always)]
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

use crate::parser::ast_node::FunctionWithArgs;
use crate::parser::ast_node::QualifiedName;
use crate::parser::ast_node::SetRestMore;
use crate::parser::ast_node::SignedNumber;
use crate::parser::ast_node::VariableTarget;
use postgres_basics::Str;
