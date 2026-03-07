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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> Option<&SchemaName> {
        self.schema.as_ref()
    }

    pub fn catalog(&self) -> Option<&str> {
        self.schema().and_then(SchemaName::catalog)
    }
}

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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn catalog(&self) -> Option<&str> {
        self.catalog.as_deref()
    }
}

use pg_basics::Str;
