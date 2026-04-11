#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RangeVar {
    relation: RelationName,
    persistence: RelationPersistence,
}

impl RangeVar {
    pub fn new(relation: RelationName) -> Self {
        Self {
            relation,
            persistence: RelationPersistence::Permanent,
        }
    }

    pub fn set_persistence(&mut self, persistence: RelationPersistence) -> &mut Self {
        self.persistence = persistence;
        self
    }

    pub fn with_persistence(mut self, persistence: RelationPersistence) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn relation(&self) -> &RelationName {
        &self.relation
    }

    pub fn persistence(&self) -> RelationPersistence {
        self.persistence
    }
}

impl From<RelationName> for RangeVar {
    fn from(relation: RelationName) -> Self {
        Self::new(relation)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RelationPersistence {
    /// regular table
    Permanent = b'p',
    /// unlogged permanent table
    Unlogged  = b'u',
    /// temporary table
    Temp      = b't',
}

use crate::RelationName;
