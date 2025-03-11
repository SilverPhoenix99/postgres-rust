#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VarList {
    Default,
    Values(Vec<VarValue>)
}

use crate::parser::ast_node::VarValue;
