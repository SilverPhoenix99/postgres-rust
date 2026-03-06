#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateDatabaseStmt {
    name: Str,
    options: Vec<CreatedbOption>,
}

impl CreateDatabaseStmt {
    pub fn new<T: Into<Str>>(name: T, options: Vec<CreatedbOption>) -> Self {
        Self {
            name: name.into(),
            options
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn options(&self) -> &[CreatedbOption] {
        &self.options
    }
}

use crate::CreatedbOption;
use pg_basics::Str;
