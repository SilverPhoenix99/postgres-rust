#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleColumnDefinition {
    name: Str,
    type_name: Type,
    collation: Option<QualifiedName>,
}

impl SimpleColumnDefinition {
    pub fn new<S, T>(name: S, type_name: T) -> Self
    where
        S: Into<Str>,
        T: Into<Type>,
    {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            collation: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_name(&self) -> &Type {
        &self.type_name
    }

    pub fn set_collation(&mut self, collation: Option<QualifiedName>) -> &mut Self {
        self.collation = collation;
        self
    }

    pub fn with_collation(mut self, collation: QualifiedName) -> Self {
        self.collation = Some(collation);
        self
    }

    pub fn collation(&self) -> Option<&[Str]> {
        self.collation.as_deref()
    }

    pub fn desconstruct(self) -> (Str, Type, Option<QualifiedName>) {
        self.into()
    }
}

impl From<SimpleColumnDefinition> for (Str, Type, Option<QualifiedName>) {
    fn from(value: SimpleColumnDefinition) -> Self {
        (value.name, value.type_name, value.collation)
    }
}

use crate::Type;
use pg_basics::QualifiedName;
use pg_basics::Str;
