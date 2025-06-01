#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateUserMappingStmt {
    user: RoleSpec,
    server: Str,
    options: Option<Vec<GenericOption>>,
    if_not_exists: bool
}

impl CreateUserMappingStmt {
    pub fn new<T: Into<Str>>(
        user: RoleSpec,
        server: T,
        options: Option<Vec<GenericOption>>,
        if_not_exists: bool
    ) -> Self {
        Self {
            user,
            server: server.into(),
            options,
            if_not_exists
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

    pub fn if_not_exists(&self) -> bool {
        self.if_not_exists
    }
}

use crate::GenericOption;
use crate::RoleSpec;
use pg_basics::Str;
