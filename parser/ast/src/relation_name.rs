#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelationName {
    name: Str,
    schema: Option<SchemaName>,
}

impl RelationName {

    pub fn new<T: Into<Str>>(name: T) -> Self {
        Self {
            name: name.into(),
            schema: None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn schema(&self) -> Option<&SchemaName> {
        self.schema.as_ref()
    }

    pub fn set_schema(&mut self, schema: Option<SchemaName>) -> &mut Self {
        self.schema = schema;
        self
    }

    pub fn with_schema<T: Into<SchemaName>>(mut self, schema: T) -> Self {
        self.schema = Some(schema.into());
        self
    }

    pub fn catalog(&self) -> Option<&str> {
        self.schema().and_then(SchemaName::catalog)
    }
}

impl From<Str> for RelationName {
    fn from(name: Str) -> Self {
        Self::new(name)
    }
}

impl From<&'static str> for RelationName {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaName {
    name: Str,
    catalog: Option<Str>
}

impl SchemaName {

    pub fn new<T: Into<Str>>(name: T) -> Self {
        Self {
            name: name.into(),
            catalog: None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn catalog(&self) -> Option<&str> {
        self.catalog.as_deref()
    }

    pub fn set_catalog(&mut self, catalog: Option<Str>) -> &mut Self {
        self.catalog = catalog;
        self
    }

    pub fn with_catalog<T: Into<Str>>(mut self, catalog: T) -> Self {
        self.catalog = Some(catalog.into());
        self
    }
}

impl From<Str> for SchemaName {
    fn from(name: Str) -> Self {
        Self::new(name)
    }
}

impl From<&'static str> for SchemaName {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

use pg_basics::Str;
