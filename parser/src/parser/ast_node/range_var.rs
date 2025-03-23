#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RangeVar {
    catalog: Option<Str>,
    schema: Option<Str>,
    relation: Str,
    persistence: RelationPersistence,
}

impl RangeVar {
    pub fn new(relation: Str) -> Self {
        Self {
            catalog: None,
            schema: None,
            relation,
            persistence: RelationPersistence::Permanent,
        }
    }

    pub fn with_schema(mut self, schema: Str) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn with_catalog(mut self, catalog: Str) -> Self {
        self.catalog = Some(catalog);
        self
    }

    pub fn with_persistence(mut self, persistence: RelationPersistence) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn relation(&self) -> &Str {
        &self.relation
    }

    pub fn catalog(&self) -> &Option<Str> {
        &self.catalog
    }

    pub fn schema(&self) -> &Option<Str> {
        &self.schema
    }

    pub fn persistence(&self) -> RelationPersistence {
        self.persistence
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

use postgres_basics::Str;
