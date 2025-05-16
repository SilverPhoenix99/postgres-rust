#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterExtensionStmt {
    name: Str,
    options: Vec<Str>,
}

impl AlterExtensionStmt {
    pub fn new<T: Into<Str>>(name: T, options: Vec<Str>) -> Self {
        Self {
            name: name.into(),
            options,
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn options(&self) -> &[Str] {
        &self.options
    }
}

use pg_basics::Str;
