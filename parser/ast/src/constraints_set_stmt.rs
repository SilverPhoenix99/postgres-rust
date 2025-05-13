#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstraintsSetStmt {
    constraints: OneOrAll<Vec<RelationName>>,
    mode: ConstraintsSetMode
}

impl ConstraintsSetStmt {
    pub fn new(constraints: OneOrAll<Vec<RelationName>>, mode: ConstraintsSetMode) -> Self {
        Self { constraints, mode }
    }

    pub fn constraints(&self) -> &OneOrAll<Vec<RelationName>> {
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

use crate::ConstraintsSetMode;
use crate::ConstraintsSetMode::Deferred;
use crate::ConstraintsSetMode::Immediate;
use crate::OneOrAll;
use crate::RelationName;
