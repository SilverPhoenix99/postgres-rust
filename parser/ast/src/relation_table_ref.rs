/// Alias: `RangeVar`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationTableRef {
    relation: RelationExpr,
    alias: Option<Alias>,
}

impl RelationTableRef {
    pub fn new<T: Into<RelationExpr>>(relation: T) -> Self {
        Self {
            relation: relation.into(),
            alias: None
        }
    }

    pub fn relation(&self) -> &RelationExpr {
        &self.relation
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

impl From<RelationExpr> for RelationTableRef {
    fn from(relation: RelationExpr) -> Self {
        Self::new(relation)
    }
}

use crate::Alias;
use crate::RelationExpr;
