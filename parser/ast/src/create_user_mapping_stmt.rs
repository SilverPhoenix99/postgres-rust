#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateUserMappingStmt {
    user: RoleSpec,
    server: Str,
    options: Option<Vec<GenericOption>>,
    existence: Presence
}

impl CreateUserMappingStmt {
    pub fn new<T: Into<Str>>(
        user: RoleSpec,
        server: T,
        options: Option<Vec<GenericOption>>,
        existence: Presence
    ) -> Self {
        Self {
            user,
            server: server.into(),
            options,
            existence
        }
    }

    pub fn user(&self) -> &RoleSpec {
        &self.user
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn options(&self) -> Option<&[GenericOption]> {
        self.options.as_deref()
    }

    pub fn existence(&self) -> Presence {
        self.existence
    }
}

use crate::GenericOption;
use crate::Presence;
use crate::RoleSpec;
use pg_basics::Str;
