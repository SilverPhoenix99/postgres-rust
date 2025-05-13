#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaName {
    name: Str,
    catalog: Option<Str>
}

impl SchemaName {

    pub fn new<T: Into<Str>>(name: T, catalog: Option<Str>) -> Self {
        Self {
            name: name.into(),
            catalog
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn catalog(&self) -> Option<&Str> {
        self.catalog.as_ref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelationName {
    name: Str,
    schema: Option<SchemaName>,
}

impl RelationName {

    pub fn new<T: Into<Str>>(name: T, schema: Option<SchemaName>) -> Self {
        Self {
            name: name.into(),
            schema
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn schema(&self) -> Option<&SchemaName> {
        self.schema.as_ref()
    }

    pub fn catalog(&self) -> Option<&Str> {
        if let Some(schema) = self.schema.as_ref() {
            return schema.catalog.as_ref()
        }
        None
    }
}

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

use postgres_basics::Str;
