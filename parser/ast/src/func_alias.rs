pub type FuncAlias = OneOrBoth<Str, Vec<FuncAliasColumn>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncAliasColumn {
    name: Str,
    type_name: Option<Type>,
    collation: Option<QualifiedName>,
}

impl FuncAliasColumn {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Str>,
    {
        Self {
            name: name.into(),
            type_name: None,
            collation: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_type_name(&mut self, type_name: Option<Type>) -> &mut Self {
        self.type_name = type_name;
        self
    }

    pub fn with_type_name<T>(mut self, type_name: T) -> Self
    where
        T: Into<Type>,
    {
        self.type_name = Some(type_name.into());
        self
    }

    pub fn type_name(&self) -> Option<&Type> {
        self.type_name.as_ref()
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
}

impl From<SimpleColumnDefinition> for FuncAliasColumn {
    fn from(value: SimpleColumnDefinition) -> Self {
        let (name, type_name, collation) = value.into();
        Self {
            name,
            type_name: Some(type_name),
            collation,
        }
    }
}

use crate::OneOrBoth;
use crate::SimpleColumnDefinition;
use crate::Type;
use pg_basics::QualifiedName;
use pg_basics::Str;
