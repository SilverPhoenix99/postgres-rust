#[derive(Debug, Clone, PartialEq)]
pub struct RangeVar {
    catalog: Option<CowStr>,
    schema: Option<CowStr>,
    relation: CowStr,
    persistence: RelationPersistence,
}

impl RangeVar {
    pub fn new(relation: CowStr) -> Self {
        Self {
            catalog: None,
            schema: None,
            relation,
            persistence: RelationPersistence::Permanent,
        }
    }

    pub fn with_schema(mut self, schema: CowStr) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn with_catalog(mut self, catalog: CowStr) -> Self {
        self.catalog = Some(catalog);
        self
    }

    pub fn with_persistence(mut self, persistence: RelationPersistence) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn relation(&self) -> &CowStr {
        &self.relation
    }

    pub fn catalog(&self) -> &Option<CowStr> {
        &self.catalog
    }

    pub fn schema(&self) -> &Option<CowStr> {
        &self.schema
    }

    pub fn persistence(&self) -> RelationPersistence {
        self.persistence
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RelationPersistence {
    /// regular table
    Permanent = b'p',
    /// unlogged permanent table
    Unlogged  = b'u',
    /// temporary table
    Temp      = b't',
}

use crate::parser::CowStr;
