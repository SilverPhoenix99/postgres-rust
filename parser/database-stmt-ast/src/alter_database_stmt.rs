#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlterDatabaseStmt {
    name: Str,
    options: Vec<AlterdbOption>
}

impl AlterDatabaseStmt {
    pub fn new<T: Into<Str>>(name: T, options: Vec<AlterdbOption>) -> Self {
        Self {
            name: name.into(),
            options
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn options(&self) -> &[AlterdbOption] {
        &self.options
    }
}

use crate::AlterdbOption;
use pg_basics::Str;
