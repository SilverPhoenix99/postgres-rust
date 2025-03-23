#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstraintsSetStmt {
    constraints: OneOrAll<Vec<RangeVar>>,
    mode: ConstraintsSetMode
}

impl ConstraintsSetStmt {
    pub fn new(constraints: OneOrAll<Vec<RangeVar>>, mode: ConstraintsSetMode) -> Self {
        Self { constraints, mode }
    }

    pub fn constraints(&self) -> &OneOrAll<Vec<RangeVar>> {
        &self.constraints
    }

    pub fn mode(&self) -> ConstraintsSetMode {
        self.mode
    }

    pub fn is_immediate(&self) -> bool {
        self.mode == Immediate
    }

    pub fn is_deferred(&self) -> bool {
        self.mode == Deferred
    }
}

use crate::parser::ast_node::ConstraintsSetMode;
use crate::parser::ast_node::ConstraintsSetMode::Deferred;
use crate::parser::ast_node::ConstraintsSetMode::Immediate;
use crate::parser::ast_node::OneOrAll;
use crate::parser::ast_node::RangeVar;
