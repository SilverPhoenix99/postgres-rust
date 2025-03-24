#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SetResetClause {
    Set(SetRest),
    Reset(VariableTarget)
}

use crate::parser::ast_node::SetRest;
use crate::parser::ast_node::VariableTarget;
