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

    pub fn with_persistence(relation: RelationName, persistence: RelationPersistence) -> Self {
        Self { relation, persistence }
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

use pg_sink_ast::RelationName;
