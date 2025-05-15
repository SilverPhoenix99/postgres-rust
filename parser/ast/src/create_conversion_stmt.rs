#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateConversionStmt {
    name: QualifiedName,
    for_encoding: Box<str>,
    to_encoding: Box<str>,
    function: QualifiedName,
    is_default: bool
}

impl CreateConversionStmt {
    pub fn new<F, T>(
        name: QualifiedName,
        for_encoding: F,
        to_encoding: T,
        function: QualifiedName,
        is_default: bool
    ) -> Self
    where
        F: Into<Box<str>>,
        T: Into<Box<str>>,
    {
        Self {
            name,
            for_encoding: for_encoding.into(),
            to_encoding: to_encoding.into(),
            function,
            is_default
        }
    }

    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    pub fn for_encoding(&self) -> &str {
        &self.for_encoding
    }

    pub fn to_encoding(&self) -> &str {
        &self.to_encoding
    }

    pub fn function(&self) -> &QualifiedName {
        &self.function
    }

    pub fn is_default(&self) -> bool {
        self.is_default
    }
}

use postgres_basics::QualifiedName;
