#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationExpr {
    name: RelationName,
    inherited: bool,
}

impl RelationExpr {
    pub fn new(name: RelationName, inherited: bool) -> Self {
        Self { name, inherited }
    }

    pub fn name(&self) -> &RelationName {
        &self.name
    }

    pub fn inherited(&self) -> bool {
        self.inherited
    }
}

use crate::RelationName;
