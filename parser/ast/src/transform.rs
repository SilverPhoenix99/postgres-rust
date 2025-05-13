#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transform {
    for_type: Type,
    language: Str
}

impl Transform {
    pub fn new<T: Into<Type>, L: Into<Str>>(for_type: T, language: L) -> Self {
        Self {
            for_type: for_type.into(),
            language: language.into()
        }
    }

    pub fn for_type(&self) -> &Type {
        &self.for_type
    }

    pub fn language(&self) -> &Str {
        &self.language
    }
}

use crate::Type;
use postgres_basics::Str;
