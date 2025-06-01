#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterExtensionStmt {
    name: Str,
    options: Option<Vec<Str>>,
}

impl AlterExtensionStmt {
    pub fn new<T: Into<Str>>(name: T, options: Option<Vec<Str>>) -> Self {
        Self {
            name: name.into(),
            options,
        }
    }

    pub fn name(&self) -> &Str {
        &self.name
    }

    pub fn options(&self) -> Option<&[Str]> {
        self.options.as_deref()
    }
}

use pg_basics::Str;
