/// Alias: `RangeVar`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationTableRef {
    relation: RelationName,
    inherited: bool,
    alias: Option<Alias>,
}

impl RelationTableRef {
    pub fn new<T: Into<RelationName>>(relation: T) -> Self {
        Self {
            relation: relation.into(),
            inherited: true,
            alias: None
        }
    }

    pub fn relation(&self) -> &RelationName {
        &self.relation
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

    pub fn set_alias(&mut self, alias: Option<Alias>) -> &mut Self {
        self.alias = alias;
        self
    }

    pub fn with_alias<T: Into<Alias>>(mut self, alias: T) -> Self {
        self.alias = Some(alias.into());
        self
    }

    pub fn alias(&self) -> Option<&Alias> {
        self.alias.as_ref()
    }
}

impl From<RelationName> for RelationTableRef {
    fn from(relation: RelationName) -> Self {
        Self::new(relation)
    }
}

use crate::Alias;
use crate::RelationName;
