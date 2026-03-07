#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SetResetClause {
    Set(SetRest),
    Reset(VariableTarget)
}

use crate::SetRest;
use crate::VariableTarget;
