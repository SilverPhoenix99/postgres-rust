#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationExpr {
    name: RelationName,
    inherited: bool,
}

impl RelationExpr {
    pub fn new<T: Into<RelationName>>(name: T) -> Self {
        Self {
            name: name.into(),
            inherited: false
        }
    }

    pub fn name(&self) -> &RelationName {
        &self.name
    }

    pub fn set_inherited(&mut self, inherited: bool) -> &mut Self {
        self.inherited = inherited;
        self
    }

    pub fn with_inherited(mut self, inherited: bool) -> Self {
        self.inherited = inherited;
        self
    }

    pub fn inherited(&self) -> bool {
        self.inherited
    }
}

impl From<RelationName> for RelationExpr {
    fn from(name: RelationName) -> Self {
        Self::new(name)
    }
}

impl From<Str> for RelationExpr {
    fn from(name: Str) -> Self {
        Self::new(name)
    }
}

impl From<&'static str> for RelationExpr {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

use crate::RelationName;
use pg_basics::Str;
